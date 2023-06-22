mod flags;
mod named;
mod tier;
mod r#type;
mod types;
mod value;

use std::fmt::{Display, Formatter};

pub use flags::Flags;
pub use named::Named;
pub use r#type::Type;
pub use tier::Tier;
pub use types::Types;
pub use value::Value;

pub(crate) fn fmt_vec<T>(f: &mut Formatter<'_>, names: &[T]) -> std::fmt::Result
where
    T: Display,
{
    write!(f, "usages:\t[ ")?;

    for (index, named) in names.iter().enumerate() {
        write!(
            f,
            "{}{}",
            named,
            if index + 1 < names.len() { ", " } else { "" }
        )?;
    }

    writeln!(f, " ]")
}
