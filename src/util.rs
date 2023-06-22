use serde::Serializer;
use std::fmt::{Display, Formatter};

pub(crate) fn as_int<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u8(u8::from(*value))
}

pub(crate) fn fmt_slice<T>(f: &mut Formatter<'_>, prefix: &str, items: &[T]) -> std::fmt::Result
where
    T: Display,
{
    write!(f, "{}:\t[ ", prefix)?;

    for (index, named) in items.iter().enumerate() {
        write!(
            f,
            "{}{}",
            named,
            if index + 1 < items.len() { ", " } else { "" }
        )?;
    }

    writeln!(f, " ]")
}

pub fn parse_bool_or_false(string: String) -> bool {
    string
        .parse::<bool>()
        .unwrap_or(string.parse::<u8>().map_or(false, |int| int != 0))
}
