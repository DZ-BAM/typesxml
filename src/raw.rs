use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename = "types")]
pub struct Types {
    #[serde(rename = "type", serialize_with = "serialize_slice_non_empty")]
    types: Option<Vec<Type>>,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Type {
    #[serde(rename = "@name")]
    name: String,
    nominal: Option<String>,
    lifetime: String,
    restock: Option<String>,
    min: Option<String>,
    quantmin: Option<String>,
    quantmax: String,
    cost: Option<String>,
    flags: Option<Flags>,
    category: Option<Named>,
    #[serde(rename = "usage", serialize_with = "serialize_optional_vec_non_empty")]
    usages: Option<Vec<Named>>,
    #[serde(rename = "value", serialize_with = "serialize_optional_vec_non_empty")]
    values: Option<Vec<Named>>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct Flags {
    #[serde(rename = "@count_in_cargo")]
    count_in_cargo: Option<String>,
    #[serde(rename = "@count_in_hoarder")]
    count_in_hoarder: Option<String>,
    #[serde(rename = "@count_in_map")]
    count_in_map: Option<String>,
    #[serde(rename = "@count_in_player")]
    count_in_player: Option<String>,
    #[serde(rename = "@crafted")]
    crafted: Option<String>,
    #[serde(rename = "@deloot")]
    deloot: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Named {
    #[serde(rename = "@name")]
    pub name: Option<String>,
}
