use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Value {
    #[serde(rename = "@name")]
    name: String,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl FromStr for Value {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            name: s.to_string(),
        })
    }
}
