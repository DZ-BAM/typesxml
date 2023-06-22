use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::fmt::{Display, Formatter};

pub(crate) fn fmt_iter<T>(f: &mut Formatter<'_>, prefix: &str, items: &[T]) -> std::fmt::Result
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

pub(crate) fn serialize_optional_vec_non_empty<T, S>(
    items: &Option<Vec<T>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    T: Display + Serialize,
    S: Serializer,
{
    serialize_slice_non_empty(items.as_deref().unwrap_or(&Vec::new()), serializer)
}

pub(crate) fn serialize_slice_non_empty<T, S>(items: &[T], serializer: S) -> Result<S::Ok, S::Error>
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
