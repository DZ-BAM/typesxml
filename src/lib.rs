use from_file::{FromFile, FromFileError};
use serde::ser::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::write;
use std::ops::Add;
use std::path::{Path, PathBuf};
use write_to_file::WriteToFile;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Types {
    #[serde(rename = "type")]
    types: Vec<Type>,
}

impl Types {
    pub fn from_xml_file(input: &str) -> Result<Self, FromFileError> {
        <Self as FromFile>::get_file_path(input)
            .and_then(<Self as FromFile>::file_read)
            .and_then(Self::from_xml_string)
    }

    pub fn from_xml_string(contents: String) -> Result<Self, FromFileError>
    where
        for<'de> Self: Deserialize<'de>,
    {
        quick_xml::de::from_str(&contents)
            .map_err(|error| FromFileError::SerdeError(error.to_string()))
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

impl FromFile for Types {
    fn from_file(input: &str) -> Result<Self, FromFileError>
    where
        for<'de> Self: Deserialize<'de> + Sized,
    {
        let pb = PathBuf::from(input);
        let ext = pb
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or(FromFileError::InvalidExtension)?;
        match ext {
            "json" => <Self as FromFile>::from_json_file(input),
            "yml" | "yaml" => <Self as FromFile>::from_yml_file(input),
            "xml" => Self::from_xml_file(input),
            _ => Err(FromFileError::InvalidExtension),
        }
    }
}

impl WriteToFile for Types {
    fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
        write(path, self.to_string())
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
