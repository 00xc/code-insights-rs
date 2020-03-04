#[macro_use]
mod validation {
    macro_rules! validate_field {
        ($self:ident, $field:ident, $limit:expr) => {
            let len = $self.$field.len();
            if len > $limit {
                return Err(Error::FieldTooLong {
                    name: stringify!($field).to_owned(),
                    len,
                    limit: $limit,
                });
            }
        };
    }

    macro_rules! validate_optional_field {
        ($self:ident, $field:ident, $limit:expr) => {
            if let Some($field) = $self.$field {
                let len = $field.len();
                if len > $limit {
                    return Err(Error::FieldTooLong {
                        name: stringify!($field).to_owned(),
                        len,
                        limit: $limit,
                    });
                }
            }
        };
    }
}

mod annotation;
mod error;
mod report;

pub use crate::annotation::*;
pub use crate::error::*;
pub use crate::report::*;
