use clap::Subcommand;
use typesxml::{Named, Type};

#[derive(Clone, Debug, Subcommand)]
pub enum FieldValue {
    #[command(long_about = "The item's name.")]
    Name { name: String },
    #[command(long_about = "Maximum amount of items of this type on the server.")]
    Nominal { nominal: Option<u8> },
    #[command(long_about = "Despawn time in seconds.")]
    Lifetime { lifetime: u32 },
    #[command(long_about = "Respawn cooldown in seconds.")]
    Restock { restock: Option<u32> },
    #[command(
        long_about = "Minimum amount for this item to spawn. Must be less than or equal to nominal."
    )]
    Min { min: u8 },
    #[command(
        long_about = "Minimum amount within the item e.g a water bottle or magazine. Use -1 if item doesnt hold a quantity of something."
    )]
    Quantmin { quantmin: Option<i64> },
    #[command(
        long_about = "Maximum amount within the item e.g a water bottle or magazine. Use -1 if item doesnt hold a quantity of something."
    )]
    Quantmax { quantmax: i64 },
    #[command(long_about = "The spawn chance, similar to a priority system.")]
    Cost { cost: Option<u32> },
    #[command(long_about = "What to take into consideration for nominal and min values.")]
    Flags {
        #[command(subcommand)]
        flags: FlagValues,
    },
    #[command(long_about = "Item category group.")]
    Category { category: Option<Named> },
    #[command(long_about = "Area for where the item will spawn e.g farm. You can have up to 4.")]
    Usages { usages: Option<Vec<Named>> },
    #[command(
        long_about = "Item value grouping. Tier1 (Spawn zones) through to Tier4 (Military)."
    )]
    Values { values: Option<Vec<Named>> },
}

impl FieldValue {
    pub fn set(&self, typ: &mut Type) {
        match self {
            Self::Name { name } => typ.set_name(name),
            Self::Nominal { nominal } => typ.set_nominal(*nominal),
            Self::Lifetime { lifetime } => typ.set_lifetime(*lifetime),
            Self::Restock { restock } => typ.set_restock(*restock),
            Self::Min { min } => typ.set_min(*min),
            Self::Quantmin { quantmin } => typ.set_quantmin(*quantmin),
            Self::Quantmax { quantmax } => typ.set_quantmax(*quantmax),
            Self::Cost { cost } => typ.set_cost(*cost),
            Self::Flags { flags } => flags.set(typ),
            Self::Category { category } => typ.set_category(category.as_ref()),
            Self::Usages { usages } => typ.set_usages(usages.as_deref()),
            Self::Values { values } => typ.set_values(values.as_deref()),
        }
    }
}

#[derive(Clone, Debug, Subcommand)]
pub enum FlagValues {
    #[command(long_about = "Includes items in cargo (backpacks, crates, cars).")]
    CountInCargo { count_in_cargo: bool },
    #[command(long_about = "Includes items in cargo (tents, barrels, stashes etc).")]
    CountInHoarder { count_in_hoarder: bool },
    #[command(long_about = "Includes items inside buildings.")]
    CountInMap { count_in_map: bool },
    #[command(long_about = "Includes items in players inventory.")]
    CountInPlayer { count_in_player: bool },
    #[command(long_about = "Item must be craftable by a player.")]
    Crafted { crafted: bool },
    #[command(long_about = "Dynamic event loot such as a heli crash.")]
    DeLoot { deloot: bool },
}

impl FlagValues {
    fn set(&self, typ: &mut Type) {
        match self {
            Self::CountInCargo { count_in_cargo } => {
                typ.mut_flags().set_count_in_cargo(*count_in_cargo);
            }
            Self::CountInHoarder { count_in_hoarder } => {
                typ.mut_flags().set_count_in_hoarder(*count_in_hoarder);
            }
            Self::CountInMap { count_in_map } => typ.mut_flags().set_count_in_map(*count_in_map),
            Self::CountInPlayer { count_in_player } => {
                typ.mut_flags().set_count_in_player(*count_in_player);
            }
            Self::Crafted { crafted } => typ.mut_flags().set_crafted(*crafted),
            Self::DeLoot { deloot } => typ.mut_flags().set_deloot(*deloot),
        }
    }
}
