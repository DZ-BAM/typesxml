use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::fmt::{Display, Formatter};

pub(crate) fn as_int<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u8(u8::from(*value))
}

pub(crate) fn as_nonempty_optional_vec<T, S>(
    items: &Option<Vec<T>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    T: Display + Serialize,
    S: Serializer,
{
    as_nonempty_slice(items.as_deref().unwrap_or(&Vec::new()), serializer)
}

pub(crate) fn as_nonempty_slice<T, S>(items: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
    T: Display + Serialize,
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(items.len()))?;

    for item in items {
        seq.serialize_element(item)?;
    }

    seq.end()
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
