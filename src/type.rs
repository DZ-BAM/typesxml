use crate::util::fmt_slice;
use crate::{raw, Flags, Named};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename = "type")]
pub struct Type {
    #[serde(rename = "@name")]
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    nominal: Option<u8>,
    lifetime: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    restock: Option<u32>,
    min: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantmin: Option<i64>,
    quantmax: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    cost: Option<u32>,
    flags: Flags,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<Named>,
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    usages: Option<Vec<Named>>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    values: Option<Vec<Named>>,
}

impl Type {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            nominal: None,
            lifetime: 0,
            restock: None,
            min: 0,
            quantmin: None,
            quantmax: 0,
            cost: None,
            flags: Flags::default(),
            category: None,
            usages: None,
            values: None,
        }
    }

    #[must_use]
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    #[must_use]
    pub fn mut_flags(&mut self) -> &mut Flags {
        &mut self.flags
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
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

    pub fn set_min(&mut self, min: u8) {
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

    pub fn set_category(&mut self, category: Option<&Named>) {
        self.category = category.cloned();
    }

    pub fn set_usages(&mut self, usages: Option<&[Named]>) {
        self.usages = usages.map(Vec::from);
    }

    pub fn set_values(&mut self, values: Option<&[Named]>) {
        self.values = values.map(Vec::from);
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "name    :\t{}", self.name)?;

        if let Some(nominal) = self.nominal {
            write!(f, "\nnominal :\t{nominal}")?;
        }

        write!(f, "\nlifetime:\t{}", self.lifetime)?;

        if let Some(restock) = self.restock {
            write!(f, "\nrestock :\t{restock}")?;
        }

        write!(f, "\nmin     :\t{}", self.min)?;

        if let Some(quantmin) = self.quantmin {
            write!(f, "\nquantmin:\t{quantmin}")?;
        }

        write!(f, "\nquantmax:\t{}", self.quantmax)?;

        if let Some(cost) = self.cost {
            write!(f, "\ncost    :\t{cost}")?;
        }

        write!(f, "\nflags   :\t{}", self.flags)?;

        if let Some(ref category) = self.category {
            write!(f, "\ncategory:\t{}", category.name())?;
        }

        if let Some(ref usages) = self.usages {
            fmt_slice(f, "\nusages  :\t", usages)?;
        }

        if let Some(ref values) = self.values {
            fmt_slice(f, "\nvalues  :\t", values)?;
        }

        Ok(())
    }
}

impl From<raw::Type> for Type {
    fn from(raw: raw::Type) -> Self {
        Self {
            name: raw.name,
            nominal: raw.nominal.and_then(|s| s.parse::<u8>().ok()),
            lifetime: raw
                .lifetime
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0),
            restock: raw.restock.and_then(|s| s.parse::<u32>().ok()),
            min: raw.min.and_then(|s| s.parse::<u8>().ok()).unwrap_or(0),
            quantmin: raw.quantmin.and_then(|s| s.parse::<i64>().ok()),
            quantmax: raw
                .quantmax
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(0),
            cost: raw.cost.and_then(|s| s.parse::<u32>().ok()),
            flags: raw.flags.map_or_else(Flags::default, Flags::from),
            category: raw
                .category
                .and_then(|category| category.name.map(Named::new)),
            usages: raw.usages.map(|usages| {
                usages
                    .into_iter()
                    .filter_map(|usage| usage.name.map(Named::new))
                    .collect()
            }),
            values: raw.values.map(|values| {
                values
                    .into_iter()
                    .filter_map(|value| value.name.map(Named::new))
                    .collect()
            }),
        }
    }
}
