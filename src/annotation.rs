use serde::{Deserialize, Serialize};

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
