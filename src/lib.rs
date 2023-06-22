use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::slice::{Iter, IterMut};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename = "types")]
pub struct Types {
    #[serde(rename = "type")]
    types: Vec<Type>,
}

impl Types {
    pub fn types(&self) -> Iter<Type> {
        self.types.iter()
    }

    pub fn mut_types(&mut self) -> IterMut<'_, Type> {
        self.types.iter_mut()
    }

    pub fn add(&mut self, typ: Type) {
        if !self.types.iter().any(|existing| existing.name == typ.name) {
            self.types.push(typ)
        }
    }

    pub fn remove(&mut self, name: &str) -> Option<Type> {
        if let Some(index) = self.types.iter().position(|typ| typ.name == name) {
            Some(self.types.remove(index))
        } else {
            None
        }
    }
}

impl Add for Types {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut map = HashMap::from(&self);
        map.extend(HashMap::from(&rhs));
        map.into()
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Type {
    #[serde(rename = "@name")]
    pub name: String,
    pub nominal: u8,
    pub lifetime: u32,
    pub restock: u32,
    pub min: u8,
    pub quantmin: i64,
    pub quantmax: i64,
    pub cost: u32,
    pub flags: Flags,
    pub category: Option<Named>,
    #[serde(rename = "usage")]
    pub usages: Option<Vec<Named>>,
    #[serde(rename = "value")]
    pub values: Option<Vec<Named>>,
}

impl Type {
    pub fn new(name: String) -> Self {
        Self {
            name,
            nominal: 0,
            lifetime: 0,
            restock: 0,
            min: 0,
            quantmin: 0,
            quantmax: 0,
            cost: 0,
            flags: Flags::default(),
            category: None,
            usages: None,
            values: None,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "name:\t{}", self.name)?;
        writeln!(f, "nominal:\t{}", self.nominal)?;
        writeln!(f, "lifetime:\t{}", self.lifetime)?;
        writeln!(f, "restock:\t{}", self.restock)?;
        writeln!(f, "min:\t{}", self.min)?;
        writeln!(f, "quantmin:\t{}", self.quantmin)?;
        writeln!(f, "quantmax:\t{}", self.quantmax)?;
        writeln!(f, "cost:\t{}", self.cost)?;
        writeln!(f, "flags:\t{}", self.flags)?;

        if let Some(category) = &self.category {
            writeln!(f, "category:\t{}", category.name)?;
        }

        if let Some(usages) = &self.usages {
            write_names(f, usages)?;
        }

        if let Some(values) = &self.values {
            write_names(f, values)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Flags {
    #[serde(rename = "@count_in_cargo")]
    pub count_in_cargo: u64,
    #[serde(rename = "@count_in_hoarder")]
    pub count_in_hoarder: u64,
    #[serde(rename = "@count_in_map")]
    pub count_in_map: u64,
    #[serde(rename = "@count_in_player")]
    pub count_in_player: u64,
    #[serde(rename = "@crafted")]
    pub crafted: u64,
    #[serde(rename = "@deloot")]
    pub deloot: u64,
}

impl Display for Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ ")?;
        write!(f, "count_in_cargo = {}, ", self.count_in_cargo)?;
        write!(f, "count_in_hoarder = {}, ", self.count_in_hoarder)?;
        write!(f, "count_in_map = {}, ", self.count_in_map)?;
        write!(f, "count_in_player = {}, ", self.count_in_player)?;
        write!(f, "crafted = {}, ", self.crafted)?;
        write!(f, "deloot = {}", self.deloot)?;
        write!(f, " ]")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Named {
    #[serde(rename = "@name")]
    pub name: String,
}

impl FromStr for Named {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            name: s.to_string(),
        })
    }
}

fn write_names(f: &mut Formatter<'_>, names: &[Named]) -> std::fmt::Result {
    write!(f, "usages:\t[ ")?;

    for (index, named) in names.iter().enumerate() {
        write!(
            f,
            "{}{}",
            named.name,
            if index + 1 < names.len() { ", " } else { "" }
        )?;
    }

    writeln!(f, " ]")
}
