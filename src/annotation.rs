use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{Error, Result};

const MESSAGE_LIMIT: usize = 2000;
const EXTERNAL_ID_LIMIT: usize = 450;

#[derive(Serialize, Deserialize, Debug)]
pub struct Annotations<'a> {
    #[serde(borrow)]
    annotations: Vec<Annotation<'a>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Severity {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    Vulnerability,
    CodeSmell,
    Bug,
}

/// A struct that represents a Code Insights annotation. Annotations enable
/// Bitbucket Server integrations to highlight specific lines to display data
/// from the result of an analysis.
///
/// It is assumed that reporters will do an analysis on the source branch of a
/// pull request, and as such might find issues on lines and files that aren't
/// changed by the pull request author. Because of this, only annotations that
/// are on lines that have been changed in a pull request are displayed.
/// Annotations can also be created on line 0 which will be displayed as a file
/// level annotation on any file that has been modified.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Annotation<'a> {
    /// The message to display to users.
    message: &'a str,

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
    path: Option<&'a str>,

    /// The line number that the annotation should belong to. If no line number
    /// is provided, then it will default to 0 and in a pull request it will
    /// appear at the top of the file specified by the path field.
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<u32>,

    /// An http or https URL representing the location of the annotation in the
    /// external tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<&'a str>,

    /// If the caller requires a link to get or modify this annotation, then an
    /// ID must be provided. It is not used or required by Bitbucket, but only
    /// by the annotation creator for updating or deleting this specific
    /// annotation.
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<&'a str>,
}

impl<'a> Annotation<'a> {
    /// Constructs a new Code Insights `Annotation` with a message and severity.
    ///
    /// The maximum length of `message` is 2000 characters. This is a Bitbucket
    /// limitation.
    pub fn new(message: &'a str, severity: Severity) -> Annotation<'a> {
        Annotation {
            message,
            severity,
            annotation_type: None,
            path: None,
            line: None,
            link: None,
            external_id: None,
        }
    }

    /// Set the annotation type.
    pub fn annotation_type(&'a mut self, annotation_type: Type) -> &'a mut Annotation {
        self.annotation_type = Some(annotation_type);
        self
    }

    /// Set the `path` to the file that is being annotated.
    ///
    /// This is the path of the file relative to the root of the Git
    /// repository. If no path is provided, then it will appear in the overview
    /// modal on all pull requests where the tip of the branch is the given
    /// commit, regardless of which files were modified.
    pub fn path(&'a mut self, path: &'a str) -> &'a mut Annotation {
        self.path = Some(path);
        self
    }

    /// Set the annotated line.
    pub fn line(&'a mut self, line: u32) -> &'a mut Annotation {
        self.line = Some(line);
        self
    }

    /// Set the annotation's link.
    ///
    /// The `link` is a URL linking to the location of the annotation in an
    /// external tool.
    pub fn link(&'a mut self, link: &'a str) -> &'a mut Annotation {
        self.link = Some(link);
        self
    }

    /// Set the annotation's external ID
    ///
    /// If the creator of the annotation requires a link to get or modify this
    /// annotation, then an ID must be provided. It is not used or required by
    /// Bitbucket, but only by the annotation creator for updating or deleting
    /// this specific annotation.
    pub fn external_id(&'a mut self, external_id: &'a str) -> &'a mut Annotation {
        self.external_id = Some(external_id);
        self
    }

    /// Serialize the annotation to a JSON `String`.
    pub fn to_string(&'a self) -> Result<String> {
        self.validate_fields()?;
        serde_json::to_string(self).map_err(Error::SerdeError)
    }

    /// Serialize the annotation to a `serde_json::Value`.
    pub fn to_value(&'a self) -> Result<Value> {
        self.validate_fields()?;
        serde_json::to_value(self).map_err(Error::SerdeError)
    }

    /// Validate fields that have limits imposed on them by Bitbucket.
    fn validate_fields(&'a self) -> Result<()> {
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
        assert!(Annotation::new(&invalid_message, Severity::Low)
            .to_value()
            .is_err());
    }

    #[test]
    fn external_id() {
        let invalid_external_id = "X".repeat(EXTERNAL_ID_LIMIT + 1);
        assert!(Annotation::new("Message", Severity::Low)
            .external_id(&invalid_external_id)
            .to_value()
            .is_err());
    }
}
