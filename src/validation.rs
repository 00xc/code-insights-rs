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

pub(crate) use validate_field;

macro_rules! validate_optional_field {
    ($self:ident, $field:ident, $limit:expr) => {
        if let Some(ref $field) = $self.$field {
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

pub(crate) use validate_optional_field;
