use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ReportResult {
    Pass,
    Fail,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data<'a> {
    /// A string describing what this data field represents.
    title: &'a str,

    #[serde(flatten)]
    #[serde(borrow)]
    parameter: Parameter<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "UPPERCASE")]
pub enum Parameter<'a> {
    Boolean(bool),
    Date(u64),
    Duration(u64),
    Link { linktext: &'a str, href: &'a str },
    Number(Number),
    Percentage(u8),
    Text(&'a str),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Report<'a> {
    /// A short string representing the name of the report.
    title: &'a str,

    /// A string to describe the purpose of the report. This string may contain
    /// escaped newlines and if it does it will display the content
    /// accordingly.
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<&'a str>,

    /// Indicates whether the report is in a passed or failed state.
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<ReportResult>,

    /// An array of data fields (described below) to display information on the
    /// report.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    data: Option<Vec<Data<'a>>>,

    /// A string to describe the tool or company who created the report.
    #[serde(skip_serializing_if = "Option::is_none")]
    reporter: Option<&'a str>,

    /// A URL linking to the results of the report in an external tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<&'a str>,

    /// A URL to the report logo. If none is provided, the default insights
    /// logo will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    logo_url: Option<&'a str>,
}

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

#[cfg(test)]
mod parameter_serialization {
    use super::*;
    use serde_json::json;

    #[test]
    fn boolean() {
        let expected = json!({"type": "BOOLEAN", "value": false});
        let actual = serde_json::to_value(Parameter::Boolean(false)).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn date() {
        let expected = json!({"type": "DATE", "value": 1582841968});
        let actual = serde_json::to_value(Parameter::Date(1582841968)).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn duration() {
        let expected = json!({"type": "DURATION", "value": 3600});
        let actual = serde_json::to_value(Parameter::Duration(3600)).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn link() {
        let expected = json!({"type": "LINK", "value": {"linktext": "Link text", "href": "https://link.test"}});
        let actual = serde_json::to_value(Parameter::Link {
            linktext: "Link text",
            href: "https://link.test",
        })
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn number() {
        let expected = json!({"type": "NUMBER", "value": 1234});
        let actual = serde_json::to_value(Parameter::Number(1234.into())).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn percentage() {
        let expected = json!({"type": "PERCENTAGE", "value": 50});
        let actual = serde_json::to_value(Parameter::Percentage(50)).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn text() {
        let expected = json!({"type": "TEXT", "value": "Some string"});
        let actual = serde_json::to_value(Parameter::Text("Some string")).unwrap();
        assert_eq!(expected, actual);
    }
}
