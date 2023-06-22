use serde::{Deserialize, Serialize, Serializer};
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
    #[serde(rename = "usage")]
    usages: Option<Vec<Named>>,
    #[serde(rename = "value")]
    values: Option<Vec<Named>>,
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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn mut_flags(&mut self) -> &mut Flags {
        &mut self.flags
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_nominal(&mut self, nominal: u8) {
        self.nominal = nominal;
    }

    pub fn set_lifetime(&mut self, lifetime: u32) {
        self.lifetime = lifetime;
    }

    pub fn set_restock(&mut self, restock: u32) {
        self.restock = restock;
    }

    pub fn set_min(&mut self, min: u8) {
        self.min = min;
    }

    pub fn set_quantmin(&mut self, quantmin: i64) {
        self.quantmin = quantmin;
    }

    pub fn set_quantmax(&mut self, quantmax: i64) {
        self.quantmax = quantmax;
    }

    pub fn set_cost(&mut self, cost: u32) {
        self.cost = cost;
    }

    pub fn set_flags(&mut self, flags: Flags) {
        self.flags = flags;
    }

    pub fn set_category(&mut self, category: Option<Named>) {
        self.category = category;
    }

    pub fn set_usages(&mut self, usages: Option<Vec<Named>>) {
        self.usages = usages;
    }

    pub fn set_values(&mut self, values: Option<Vec<Named>>) {
        self.values = values;
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
    #[serde(rename = "@count_in_cargo", serialize_with = "as_int")]
    count_in_cargo: bool,
    #[serde(rename = "@count_in_hoarder", serialize_with = "as_int")]
    count_in_hoarder: bool,
    #[serde(rename = "@count_in_map", serialize_with = "as_int")]
    count_in_map: bool,
    #[serde(rename = "@count_in_player", serialize_with = "as_int")]
    count_in_player: bool,
    #[serde(rename = "@crafted", serialize_with = "as_int")]
    crafted: bool,
    #[serde(rename = "@deloot", serialize_with = "as_int")]
    deloot: bool,
}

impl Flags {
    pub fn set_count_in_cargo(&mut self, count_in_cargo: bool) {
        self.count_in_cargo = count_in_cargo;
    }

    pub fn set_count_in_hoarder(&mut self, count_in_hoarder: bool) {
        self.count_in_hoarder = count_in_hoarder;
    }

    pub fn set_count_in_map(&mut self, count_in_map: bool) {
        self.count_in_map = count_in_map;
    }

    pub fn set_count_in_player(&mut self, count_in_player: bool) {
        self.count_in_player = count_in_player;
    }

    pub fn set_crafted(&mut self, crafted: bool) {
        self.crafted = crafted;
    }

    pub fn set_deloot(&mut self, deloot: bool) {
        self.deloot = deloot;
    }
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

fn as_int<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u8(u8::from(*value))
}
