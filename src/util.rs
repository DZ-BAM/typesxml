use serde::Serializer;
use std::fmt::{Display, Formatter};

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn as_int<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u8(u8::from(*value))
}

pub fn fmt_slice<T>(f: &mut Formatter<'_>, prefix: &str, items: &[T]) -> std::fmt::Result
where
    T: Display,
{
    write!(f, "{prefix}[ ")?;

    for (index, item) in items.iter().enumerate() {
        write!(
            f,
            "{}{}",
            item,
            if index + 1 < items.len() { ", " } else { "" }
        )?;
    }

    write!(f, " ]")
}

pub fn parse_bool_or_false(string: &str) -> bool {
    string
        .parse::<bool>()
        .unwrap_or_else(|_| string.parse::<u8>().map_or(false, |int| int != 0))
}
