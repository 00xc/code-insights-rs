use std::fmt;

/// Provides descriptive errors when the serialization of a `Report` or
/// `Annotation` fails.
#[derive(Debug)]
pub enum Error {
    FieldTooLong {
        name: String,
        len: usize,
        limit: usize,
    },
    SerdeError(serde_json::Error),
}

/// Shorthand for [`Result`] type.
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::FieldTooLong {
                ref name,
                len,
                limit,
            } => write!(
                f,
                "field '{}' too long, its length {} is longer than the allowed limit {}",
                name, len, limit
            ),
            Error::SerdeError(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::FieldTooLong { .. } => None,
            Error::SerdeError(ref e) => Some(e),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::SerdeError(err)
    }
}
