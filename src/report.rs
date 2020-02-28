use serde::{Deserialize, Serialize};
use serde_json::{Number, Result, Value};

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

/// A struct that represents a Bitbucket Server [Code Insights](https://confluence.atlassian.com/bitbucketserver/code-insights-966660485.html)
/// report.
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

impl<'a> Report<'a> {
    /// Constructs a new Code Insights `Report` with the title `title`.
    ///
    /// The maximum length of `title` is 450 characters. This is a Bitbucket
    /// limitation. It is recommended to use a short title for display purposes
    /// in Bitbucket.
    pub fn new(title: &'a str) -> Report<'a> {
        Report {
            title,
            details: None,
            result: None,
            data: None,
            reporter: None,
            link: None,
            logo_url: None,
        }
    }

    /// Sets the report's details.
    ///
    /// The report details are intended to describe the purpose of the report.
    /// It may contain escaped newlines and if it does, Bitbucket will display
    /// the content accordingly.
    ///
    /// The maximum length of `details` is 2000 characters. This is a Bitbucket
    /// limitation.
    pub fn details(&'a mut self, details: &'a str) -> &'a mut Report {
        self.details = Some(details);
        self
    }

    /// Sets the result of the `Report` which indicates whether the report is
    /// in a passed or failed state.
    pub fn result(&'a mut self, result: ReportResult) -> &'a mut Report {
        self.result = Some(result);
        self
    }

    /// Sets the data fields, which are used to display information related to
    /// the report.
    ///
    /// Examples of data fields may be code coverage percentage or the number
    /// of linter errors.
    ///
    /// A maximum of 6 `data` fields are allowed. This is a Bitbucket
    /// limitation.
    pub fn data(&'a mut self, data: Vec<Data<'a>>) -> &'a mut Report {
        self.data = Some(data);
        self
    }

    /// Sets the reporter.
    ///
    /// The reporter describes the tool or company which created the Code
    /// Insights report.
    ///
    /// The maximum length of `reporter` is 450 characters. This is a Bitbucket
    /// limitation.
    pub fn reporter(&'a mut self, reporter: &'a str) -> &'a mut Report {
        self.reporter = Some(reporter);
        self
    }

    /// Set the report's link.
    ///
    /// The `link` is a URL linking to the results of the report in an external
    /// tool.
    pub fn link(&'a mut self, link: &'a str) -> &'a mut Report {
        self.link = Some(link);
        self
    }

    /// Set the report's logo URL.
    ///
    /// The report logo will be displayed by Bitbucket when the report is
    /// presented to the user. It is recommended to use an SVG logo.
    pub fn logo_url(&'a mut self, logo_url: &'a str) -> &'a mut Report {
        self.logo_url = Some(logo_url);
        self
    }

    /// Serialize the report to a JSON `String`.
    pub fn to_string(&'a self) -> Result<String> {
        serde_json::to_string(self)
    }

    /// Serialize the report to a `serde_json::Value`.
    pub fn to_value(&'a self) -> Result<Value> {
        serde_json::to_value(self)
    }
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
