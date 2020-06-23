use thiserror::Error;

/// Provides descriptive errors when the serialization of a `Report` or
/// `Annotation` fails.
#[derive(Debug, Error)]
pub enum Error {
    #[error("field '{name}' too long, its length {len} is longer than the allowed limit {limit}")]
    FieldTooLong {
        name: String,
        len: usize,
        limit: usize,
    },
    #[error("serialization error")]
    SerdeError(#[from] serde_json::Error),
}

/// Shorthand for [`Result`] type.
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
pub type Result<T> = std::result::Result<T, Error>;
