use crate::{raw, Type};
use serde::{Deserialize, Serialize};
use serde_rw::{Error, FromFile};
use std::ops::Add;
use std::slice::{Iter, IterMut};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename = "types")]
pub struct Types {
    #[serde(rename = "type")]
    types: Vec<Type>,
}

impl Types {
    /// Parse the types.xml gracefully
    ///
    /// This corrects common formatting errors but may also lead to data loss.
    ///
    /// # Arguments
    /// * `filename` - The path to the file to read.
    ///
    /// # Errors
    /// Returns a `serde::rw::Error` if the deserialization fails.
    pub fn read_gracefully(filename: &str) -> Result<Self, Error> {
        raw::Types::from_file(filename).map(Self::from)
    }

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
            self.types.push(typ);
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
        let mut types = self.types;
        types.extend(rhs.types);
        types.sort_by(|lhs, rhs| lhs.get_name().cmp(rhs.get_name()));
        types.dedup_by(|lhs, rhs| lhs.get_name().eq(rhs.get_name()));
        Self { types }
    }
}

impl From<raw::Types> for Types {
    fn from(raw: raw::Types) -> Self {
        Self {
            types: raw.types.map_or(Vec::new(), |types| {
                types.into_iter().map(Type::from).collect()
            }),
        }
    }
}

impl From<Vec<Type>> for Types {
    fn from(types: Vec<Type>) -> Self {
        Self { types }
    }
}
