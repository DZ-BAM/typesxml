use crate::util::{fmt_slice, serialize_optional_vec_non_empty};
use crate::{Flags, Named};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Type {
    #[serde(rename = "@name")]
    name: String,
    nominal: Option<u8>,
    lifetime: u32,
    restock: Option<u32>,
    min: Option<u8>,
    quantmin: Option<i64>,
    quantmax: i64,
    cost: Option<u32>,
    flags: Flags,
    category: Option<Named>,
    #[serde(rename = "usage", serialize_with = "serialize_optional_vec_non_empty")]
    usages: Option<Vec<Named>>,
    #[serde(rename = "value", serialize_with = "serialize_optional_vec_non_empty")]
    values: Option<Vec<Named>>,
}

impl Type {
    pub fn new(name: String) -> Self {
        Self {
            name,
            nominal: None,
            lifetime: 0,
            restock: None,
            min: None,
            quantmin: None,
            quantmax: 0,
            cost: None,
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

    pub fn set_nominal(&mut self, nominal: Option<u8>) {
        self.nominal = nominal;
    }

    pub fn set_lifetime(&mut self, lifetime: u32) {
        self.lifetime = lifetime;
    }

    pub fn set_restock(&mut self, restock: Option<u32>) {
        self.restock = restock;
    }

    pub fn set_min(&mut self, min: Option<u8>) {
        self.min = min;
    }

    pub fn set_quantmin(&mut self, quantmin: Option<i64>) {
        self.quantmin = quantmin;
    }

    pub fn set_quantmax(&mut self, quantmax: i64) {
        self.quantmax = quantmax;
    }

    pub fn set_cost(&mut self, cost: Option<u32>) {
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

        if let Some(nominal) = self.nominal {
            writeln!(f, "nominal:\t{}", nominal)?;
        }

        writeln!(f, "lifetime:\t{}", self.lifetime)?;

        if let Some(restock) = self.restock {
            writeln!(f, "restock:\t{}", restock)?;
        }

        if let Some(min) = self.min {
            writeln!(f, "min:\t{}", min)?;
        }

        if let Some(quantmin) = self.quantmin {
            writeln!(f, "quantmin:\t{}", quantmin)?;
        }

        writeln!(f, "quantmax:\t{}", self.quantmax)?;

        if let Some(cost) = self.cost {
            writeln!(f, "cost:\t{}", cost)?;
        }

        writeln!(f, "flags:\t{}", self.flags)?;

        if let Some(category) = &self.category {
            writeln!(f, "category:\t{}", category.name)?;
        }

        if let Some(usages) = &self.usages {
            fmt_slice(f, "usages", usages)?;
        }

        if let Some(values) = &self.values {
            fmt_slice(f, "values", values)?;
        }

        Ok(())
    }
}
