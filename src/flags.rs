use crate::raw;
use crate::util::{as_int, parse_bool_or_false};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
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

impl From<raw::Flags> for Flags {
    fn from(raw: raw::Flags) -> Self {
        Self {
            count_in_cargo: raw
                .count_in_cargo
                .map_or(false, |s| parse_bool_or_false(&s)),
            count_in_hoarder: raw
                .count_in_hoarder
                .map_or(false, |s| parse_bool_or_false(&s)),
            count_in_map: raw.count_in_map.map_or(false, |s| parse_bool_or_false(&s)),
            count_in_player: raw
                .count_in_player
                .map_or(false, |s| parse_bool_or_false(&s)),
            crafted: raw.crafted.map_or(false, |s| parse_bool_or_false(&s)),
            deloot: raw.deloot.map_or(false, |s| parse_bool_or_false(&s)),
        }
    }
}
