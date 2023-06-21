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
        let mut types: Vec<Type> = map.into_values().cloned().collect();
        types.sort_by(|lhs, rhs| lhs.name.cmp(&rhs.name));
        Self { types }
    }
}

impl Display for Types {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<types>{}</types>",
            self.types
                .iter()
                .map(|typ| typ.to_string())
                .collect::<String>()
        )
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
    type Err = serde_xml_rs::Error;

    fn from_str(xml: &str) -> Result<Self, Self::Err> {
        serde_xml_rs::from_str(xml)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Type {
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

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"<type name="{}">"#, self.name)?;
        write!(f, "<nominal>{}</nominal>", self.nominal)?;
        write!(f, "<lifetime>{}</lifetime>", self.lifetime)?;
        write!(f, "<restock>{}</restock>", self.restock)?;
        write!(f, "<min>{}</min>", self.min)?;
        write!(f, "<quantmin>{}</quantmin>", self.quantmin)?;
        write!(f, "<quantmax>{}</quantmax>", self.quantmax)?;
        write!(f, "<cost>{}</cost>", self.cost)?;
        write!(f, "{}", self.flags)?;

        if let Some(category) = &self.category {
            write!(f, r#"<category name="{}"/>"#, category.name)?;
        }

        if let Some(usages) = &self.usage {
            for usage in usages {
                write!(f, r#"<usage name="{}"/>"#, usage.name)?;
            }
        }

        if let Some(values) = &self.value {
            for value in values {
                write!(f, r#"<value name="{}"/>"#, value.name)?;
            }
        }

        write!(f, "</type>")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Flags {
    count_in_cargo: u64,
    count_in_hoarder: u64,
    count_in_map: u64,
    count_in_player: u64,
    crafted: u64,
    deloot: u64,
}

impl Display for Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<flags count_in_cargo="{}" count_in_hoarder="{}" count_in_map="{}" count_in_player="{}" crafted="{}" deloot="{}"/>"#,
            self.count_in_cargo,
            self.count_in_hoarder,
            self.count_in_map,
            self.count_in_player,
            self.crafted,
            self.deloot
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Named {
    #[serde(rename = "name")]
    name: String,
}
