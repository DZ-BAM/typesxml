use clap::Subcommand;
use typesxml::Named;

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
