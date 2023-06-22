use crate::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Add;
use std::slice::{Iter, IterMut};

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
        if !self
            .types
            .iter()
            .any(|existing| existing.get_name() == typ.get_name())
        {
            self.types.push(typ)
        }
    }

    pub fn remove(&mut self, name: &str) -> Option<Type> {
        if let Some(index) = self.types.iter().position(|typ| typ.get_name() == name) {
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
        types.sort_by(|lhs, rhs| lhs.get_name().cmp(rhs.get_name()));
        types.into()
    }
}

impl<'a> From<&'a Types> for HashMap<&'a str, &'a Type> {
    fn from(types: &'a Types) -> Self {
        types
            .types
            .iter()
            .map(|typ| (typ.get_name(), typ))
            .collect()
    }
}
