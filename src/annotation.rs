use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{Error, Result};
use crate::validation::{validate_field, validate_optional_field};

const MESSAGE_LIMIT: usize = 2000;
const EXTERNAL_ID_LIMIT: usize = 450;

/// Holds all annotations that apply to a Code Insights report.
///
/// A Code Insights report must have been created in Bitbucket Server before
/// any annotations can be posted, and a report cannot have more than 1000
/// annotations by default.
///
/// This is the struct that should be serialized and POST:ed to Bitbucket
/// Server's annotations endpoint.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Annotations {
    annotations: Vec<Annotation>,
}

impl Annotations {
    pub fn new<T: Into<Vec<Annotation>>>(annotations: T) -> Self {
        Annotations {
            annotations: annotations.into(),
        }
    }
}

/// Represents the severity of an `Annotation`.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Severity {
    Low,
    Medium,
    High,
}

/// Represents the type of an `Annotation`.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    Vulnerability,
    CodeSmell,
    Bug,
}

/// Represents a Code Insights annotation. Annotations enable Bitbucket Server
/// integrations to highlight specific lines to display data from the result of
/// an analysis.
///
/// It is assumed that reporters will do an analysis on the source branch of a
/// pull request, and as such might find issues on lines and files that aren't
/// changed by the pull request author. Because of this, only annotations that
/// are on lines that have been changed in a pull request are displayed.
/// Annotations can also be created on line 0 which will be displayed as a file
/// level annotation on any file that has been modified.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    /// The message to display to users.
    message: String,

    /// The severity of the annotation.
    severity: Severity,

    /// The type of annotation posted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    annotation_type: Option<Type>,

    /// The path of the file on which this annotation should be placed. This is
    /// the path of the file relative to the git repository. If no path is
    /// provided, then it will appear in the overview modal on all pull
    /// requests where the tip of the branch is the given commit, regardless of
    /// which files were modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,

    /// The line number that the annotation should belong to. If no line number
    /// is provided, then it will default to 0 and in a pull request it will
    /// appear at the top of the file specified by the path field.
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<u32>,

    /// An http or https URL representing the location of the annotation in the
    /// external tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<String>,

    /// If the caller requires a link to get or modify this annotation, then an
    /// ID must be provided. It is not used or required by Bitbucket, but only
    /// by the annotation creator for updating or deleting this specific
    /// annotation.
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<String>,
}

impl Annotation {
    /// Validates fields that have limits imposed on them by Bitbucket.
    fn validate_fields(&self) -> Result<()> {
        validate_field!(self, message, MESSAGE_LIMIT);
        validate_optional_field!(self, external_id, EXTERNAL_ID_LIMIT);
        Ok(())
    }
}

impl TryFrom<Annotation> for String {
    type Error = Error;

    fn try_from(value: Annotation) -> std::result::Result<Self, Self::Error> {
        value.validate_fields()?;
        serde_json::to_string(&value).map_err(Error::SerdeError)
    }
}

impl TryFrom<Annotation> for Value {
    type Error = Error;

    fn try_from(value: Annotation) -> std::result::Result<Self, Self::Error> {
        value.validate_fields()?;
        serde_json::to_value(value).map_err(Error::SerdeError)
    }
}

pub struct AnnotationBuilder {
    message: String,
    severity: Severity,
    annotation_type: Option<Type>,
    path: Option<String>,
    line: Option<u32>,
    link: Option<String>,
    external_id: Option<String>,
}

impl AnnotationBuilder {
    /// Constructs a new Code Insights `Annotation` with a message and severity.
    ///
    /// The maximum length of `message` is 2000 characters. This is a Bitbucket
    /// limitation.
    pub fn new<T: Into<String>>(message: T, severity: Severity) -> Self {
        AnnotationBuilder {
            message: message.into(),
            severity,
            annotation_type: None,
            path: None,
            line: None,
            link: None,
            external_id: None,
        }
    }

    /// Sets the annotation type.
    pub fn annotation_type(mut self, annotation_type: Type) -> Self {
        self.annotation_type = Some(annotation_type);
        self
    }

    /// Sets the path to the file that is being annotated.
    ///
    /// This is the path of the file relative to the root of the Git
    /// repository. If no path is provided, then it will appear in the overview
    /// modal on all pull requests where the tip of the branch is the given
    /// commit, regardless of which files were modified.
    pub fn path<T: Into<String>>(mut self, path: T) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Sets the annotated line.
    ///
    /// If no line is set, the annotation will displayed as an annotation that
    /// applies to the whole file.
    pub fn line(mut self, line: u32) -> Self {
        self.line = Some(line);
        self
    }

    /// Sets the annotation's link.
    ///
    /// The link is the location of the annotation in an external tool.
    pub fn link<T: Into<String>>(mut self, link: T) -> Self {
        self.link = Some(link.into());
        self
    }

    /// Sets the annotation's external ID
    ///
    /// If the creator of the annotation requires a link to get or modify this
    /// annotation, then an ID must be provided. It is not used or required by
    /// Bitbucket, but only by the annotation creator for updating or deleting
    /// this specific annotation.
    pub fn external_id<T: Into<String>>(mut self, external_id: T) -> Self {
        self.external_id = Some(external_id.into());
        self
    }

    /// Create the annotation
    ///
    /// # Errors
    ///
    /// Will return `Err` if `message` or `external_id` are longer than the
    /// Bitbucket API allows.
    pub fn build(self) -> Result<Annotation> {
        self.validate_fields()?;

        let AnnotationBuilder {
            message,
            severity,
            annotation_type,
            path,
            line,
            link,
            external_id,
        } = self;

        Ok(Annotation {
            message,
            severity,
            annotation_type,
            path,
            line,
            link,
            external_id,
        })
    }

    /// Validates fields that have limits imposed on them by Bitbucket.
    fn validate_fields(&self) -> Result<()> {
        validate_field!(self, message, MESSAGE_LIMIT);
        validate_optional_field!(self, external_id, EXTERNAL_ID_LIMIT);
        Ok(())
    }
}

#[cfg(test)]
mod field_validataion {
    use super::*;

    #[test]
    fn message() {
        let invalid_message = "X".repeat(MESSAGE_LIMIT + 1);
        assert!(AnnotationBuilder::new(invalid_message, Severity::Low)
            .build()
            .is_err());
    }

    #[test]
    fn external_id() {
        let invalid_external_id = "X".repeat(EXTERNAL_ID_LIMIT + 1);
        assert!(AnnotationBuilder::new("Message", Severity::Low)
            .external_id(invalid_external_id)
            .build()
            .is_err());
    }
}
