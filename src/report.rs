use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use crate::error::{Error, Result};

const TITLE_LIMIT: usize = 450;
const DETAILS_LIMIT: usize = 2000;
const DATA_LIMIT: usize = 6;
const REPORTER_LIMIT: usize = 450;

/// Indicates whether a `Report` is in a passed or failed state.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ReportResult {
    Pass,
    Fail,
}

/// Used to represent a data field in a `Report`.
///
/// A data field contains information that will be displayed in the Code
/// Insights report summary in Bitbucket Server..
#[derive(Serialize, Deserialize, Debug)]
pub struct Data<'a> {
    /// A string describing what this data field represents.
    title: &'a str,

    /// The value of the data field.
    #[serde(flatten)]
    #[serde(borrow)]
    parameter: Parameter<'a>,
}

/// Describes the value for a `Data` field in a `Report`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "UPPERCASE")]
pub enum Parameter<'a> {
    /// The value will be displayed as 'Yes' or 'No'.
    Boolean(bool),

    /// The value is in the form of a Unix timestamp (milliseconds) and will be
    /// displayed as a relative date if the date is less than one week ago,
    /// otherwise as an absolute date.
    Date(u64),

    /// The value is a duration in milliseconds and will be displayed in a
    /// human readable duration format.
    Duration(u64),

    /// The value will be displayed as a clickable link with the text
    /// `linktext`.
    Link { linktext: &'a str, href: &'a str },

    /// The value is a JSON number and large numbers will be displayed in a
    /// human readable format (e.g. 14.3k).
    Number(Number),

    /// The value is a number between 0 and 100 and will be displayed with a
    /// percentage sign.
    Percentage(u8),

    /// The value is text that will be displayed as-is.
    Text(&'a str),
}

/// Represents a Bitbucket Server Code Insights report.
///
/// Reports enable Bitbucket Server integrations to give a high-level overview
/// of the results of the analysis and display data that is not specific to any
/// given file. A report must be created before any annotations are able to be
/// created as annotations must be associated with an existing report.
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
    pub fn new(title: &'a str) -> Self {
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
    pub fn details(&'a mut self, details: &'a str) -> &'a mut Self {
        self.details = Some(details);
        self
    }

    /// Sets the result of the `Report` which indicates whether the report is
    /// in a passed or failed state.
    pub fn result(&'a mut self, result: ReportResult) -> &'a mut Self {
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
    pub fn data(&'a mut self, data: Vec<Data<'a>>) -> &'a mut Self {
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
    pub fn reporter(&'a mut self, reporter: &'a str) -> &'a mut Self {
        self.reporter = Some(reporter);
        self
    }

    /// Sets the report's link.
    ///
    /// The `link` is a URL linking to the results of the report in an external
    /// tool.
    pub fn link(&'a mut self, link: &'a str) -> &'a mut Self {
        self.link = Some(link);
        self
    }

    /// Sets the report's logo URL.
    ///
    /// The report logo will be displayed by Bitbucket when the report is
    /// presented to the user. It is recommended to use an SVG logo.
    pub fn logo_url(&'a mut self, logo_url: &'a str) -> &'a mut Self {
        self.logo_url = Some(logo_url);
        self
    }

    /// Serializes the report to a JSON `String`.
    pub fn to_string(&'a self) -> Result<String> {
        self.validate_fields()?;
        serde_json::to_string(self).map_err(Error::SerdeError)
    }

    /// Serializes the report to a `serde_json::Value`.
    pub fn to_value(&'a self) -> Result<Value> {
        self.validate_fields()?;
        serde_json::to_value(self).map_err(Error::SerdeError)
    }

    /// Validates fields that have limits imposed on them by Bitbucket.
    fn validate_fields(&'a self) -> Result<()> {
        validate_field!(self, title, TITLE_LIMIT);
        validate_optional_field!(self, details, DETAILS_LIMIT);
        validate_optional_field!(self, reporter, REPORTER_LIMIT);

        if let Some(data) = &self.data {
            let len = data.len();
            if len > DATA_LIMIT {
                return Err(Error::FieldTooLong {
                    name: "data".to_owned(),
                    len,
                    limit: DATA_LIMIT,
                });
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod field_validation {
    use super::*;

    #[test]
    fn title() {
        let invalid_title = "X".repeat(TITLE_LIMIT + 1);
        assert!(Report::new(&invalid_title).to_value().is_err());
    }

    #[test]
    fn details() {
        let invalid_detail = "X".repeat(DETAILS_LIMIT + 1);
        assert!(Report::new("Title")
            .details(&invalid_detail)
            .to_value()
            .is_err());
    }

    #[test]
    fn reporter() {
        let invalid_reporter = "X".repeat(REPORTER_LIMIT + 1);
        assert!(Report::new("Title")
            .reporter(&invalid_reporter)
            .to_value()
            .is_err());
    }

    #[test]
    fn data() {
        let mut data = Vec::new();

        for _ in 0..=DATA_LIMIT {
            data.push(Data {
                title: "Title",
                parameter: Parameter::Boolean(true),
            });
        }
        assert!(Report::new("Title").data(data).to_value().is_err());
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
