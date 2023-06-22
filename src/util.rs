use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
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

pub(crate) fn nonempty<T, S>(option: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(if option.is_some() { Some(1) } else { None })?;

    if let Some(item) = option {
        seq.serialize_element(item)?;
    }

    seq.end()
}

pub fn parse_bool_or_false(string: String) -> bool {
    string
        .parse::<bool>()
        .unwrap_or(string.parse::<u8>().map_or(false, |int| int != 0))
}
