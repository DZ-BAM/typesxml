use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Tier {
    Tier1,
    Tier2,
    Tier3,
    Tier4,
}

impl Display for Tier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Tier1 => "Tier1",
                Self::Tier2 => "Tier2",
                Self::Tier3 => "Tier3",
                Self::Tier4 => "Tier4",
            }
        )
    }
}

impl FromStr for Tier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Tier1" => Ok(Self::Tier1),
            "Tier2" => Ok(Self::Tier2),
            "Tier3" => Ok(Self::Tier3),
            "Tier4" => Ok(Self::Tier4),
            _ => Err(format!("Invalid tier: {}", s)),
        }
    }
}
