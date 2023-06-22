use crate::{Flags, Named, Value};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
    values: Option<Vec<Value>>,
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

    pub fn set_values(&mut self, values: Option<Vec<Value>>) {
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
            fmt_iter(f, "usages", usages)?;
        }

        if let Some(values) = &self.values {
            fmt_iter(f, "values", values)?;
        }

        Ok(())
    }
}

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
