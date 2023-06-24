mod flags;
mod named;
mod raw;
mod r#type;
mod types;
mod util;

pub use flags::Flags;
pub use named::Named;
pub use r#type::Type;
pub use types::Types;

pub const XML_INDENT_CHAR: char = ' ';
pub const XML_INDENT_SIZE: usize = 4;
