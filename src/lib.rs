use serde::ser::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Types {
    #[serde(rename = "type")]
    types: Vec<Type>,
}

impl Add for Types {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut map = HashMap::from(&self);
        map.extend(HashMap::from(&rhs));
        map.into()
    }
}

impl Display for Types {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        quick_xml::se::to_writer(f, self).map_err(std::fmt::Error::custom)
    }
}

impl From<Vec<Type>> for Types {
    fn from(types: Vec<Type>) -> Self {
        Self { types }
    }
}

impl<'a> From<HashMap<&'a str, &'a Type>> for Types {
    fn from(types: HashMap<&'a str, &'a Type>) -> Self {
        let mut types: Vec<Type> = types.into_values().cloned().collect();
        types.sort_by(|lhs, rhs| lhs.name.cmp(&rhs.name));
        types.into()
    }
}

impl<'a> From<&'a Types> for HashMap<&'a str, &'a Type> {
    fn from(types: &'a Types) -> Self {
        types
            .types
            .iter()
            .map(|typ| (typ.name.as_str(), typ))
            .collect()
    }
}

impl FromStr for Types {
    type Err = quick_xml::de::DeError;

    fn from_str(xml: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(xml)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Type {
    #[serde(rename = "@name")]
    name: String,
    nominal: u8,
    lifetime: u32,
    restock: u32,
    min: u8,
    quantmin: i64,
    quantmax: i64,
    cost: u32,
    flags: Flags,
    category: Option<Named>,
    usage: Option<Vec<Named>>,
    value: Option<Vec<Named>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Flags {
    #[serde(rename = "@count_in_cargo")]
    count_in_cargo: u64,
    #[serde(rename = "@count_in_hoarder")]
    count_in_hoarder: u64,
    #[serde(rename = "@count_in_map")]
    count_in_map: u64,
    #[serde(rename = "@count_in_player")]
    count_in_player: u64,
    #[serde(rename = "@crafted")]
    crafted: u64,
    #[serde(rename = "@deloot")]
    deloot: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Named {
    #[serde(rename = "@name")]
    name: String,
}
