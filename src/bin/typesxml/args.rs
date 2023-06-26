use clap::{Args, Parser, Subcommand};
use regex::Regex;
use typesxml::Named;

const DESCRIPTION: &str = "Manipulate types.xml files for DayZ servers.";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = DESCRIPTION)]
pub struct Arguments {
    #[arg(index = 1)]
    pub(crate) file: String,
    #[command(subcommand)]
    pub(crate) action: Action,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Action {
    #[command(long_about = "Add a new type")]
    Add(Add),
    #[command(long_about = "Display the selected type's properties")]
    Find(Find),
    #[command(long_about = "Fix errors in the given file")]
    Fix,
    #[command(long_about = "Merge an extension XML file into the base XML file")]
    Merge(Merge),
    #[command(long_about = "Remove an existing type")]
    Remove(Remove),
    #[command(long_about = "Set the selected type's properties")]
    Set(Set),
    #[command(long_about = "Show the selected type")]
    Show(Show),
}

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

#[derive(Clone, Debug, Args)]
pub struct Add {
    #[arg(index = 1, name = "type")]
    pub(crate) name: String,
    #[arg(long, short, help = "Write result to the given file instead of STDOUT")]
    pub(crate) output: Option<String>,
    #[arg(long, short, help = "Write result to the original file")]
    pub(crate) in_place: bool,
}

#[derive(Clone, Debug, Args)]
pub struct Find {
    #[arg(index = 1, name = "type")]
    pub(crate) regex: Regex,
}

#[derive(Clone, Debug, Args)]
pub struct Merge {
    #[arg(index = 1)]
    pub(crate) extension: String,
    #[arg(long, short, help = "Write result to the given file instead of STDOUT")]
    pub(crate) output: Option<String>,
}

#[derive(Clone, Debug, Args)]
pub struct Remove {
    #[arg(index = 1, name = "type")]
    pub(crate) name: String,
    #[arg(long, short)]
    pub(crate) output: Option<String>,
    #[arg(long, short)]
    pub(crate) in_place: bool,
}

#[derive(Clone, Debug, Args)]
pub struct Set {
    #[arg(index = 1, name = "type")]
    pub(crate) name: String,
    #[command(subcommand)]
    pub(crate) field_value: FieldValue,
    #[arg(long, short, help = "Write result to the given file instead of STDOUT")]
    pub(crate) output: Option<String>,
    #[arg(long, short, help = "Write result to the original file")]
    pub(crate) in_place: bool,
}

#[derive(Clone, Debug, Args)]
pub struct Show {
    #[arg(index = 1, name = "type")]
    pub(crate) name: String,
    #[arg(long, short, help = "Show type as XML")]
    pub(crate) xml: bool,
}
