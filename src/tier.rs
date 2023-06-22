use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Tier {
    Tier1,
    Tier2,
    Tier3,
    Tier4,
}

impl<'de> Deserialize<'de> for Tier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TierVisitor;

        impl<'de> Visitor<'de> for TierVisitor {
            type Value = Tier;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("Tier1, Tier2, Tier3 or Tier4")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Tier::from_str(value).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(TierVisitor)
    }
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

impl Serialize for Tier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
