use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename = "types")]
pub struct Types {
    #[serde(rename = "type")]
    pub(crate) types: Option<Vec<Type>>,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Type {
    #[serde(rename = "@name")]
    pub(crate) name: String,
    pub(crate) nominal: Option<String>,
    pub(crate) lifetime: Option<String>,
    pub(crate) restock: Option<String>,
    pub(crate) min: Option<String>,
    pub(crate) quantmin: Option<String>,
    pub(crate) quantmax: Option<String>,
    pub(crate) cost: Option<String>,
    pub(crate) flags: Option<Flags>,
    pub(crate) category: Option<Named>,
    #[serde(rename = "usage")]
    pub(crate) usages: Option<Vec<Named>>,
    #[serde(rename = "value")]
    pub(crate) values: Option<Vec<Named>>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct Flags {
    #[serde(rename = "@count_in_cargo")]
    pub(crate) count_in_cargo: Option<String>,
    #[serde(rename = "@count_in_hoarder")]
    pub(crate) count_in_hoarder: Option<String>,
    #[serde(rename = "@count_in_map")]
    pub(crate) count_in_map: Option<String>,
    #[serde(rename = "@count_in_player")]
    pub(crate) count_in_player: Option<String>,
    #[serde(rename = "@crafted")]
    pub(crate) crafted: Option<String>,
    #[serde(rename = "@deloot")]
    pub(crate) deloot: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Named {
    #[serde(rename = "@name")]
    pub(crate) name: Option<String>,
}
