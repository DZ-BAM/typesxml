use clap::{Parser, Subcommand};
use serde_rw::{FromFile, ToFile};
use std::process::exit;
use typesxml::{Named, Type, Types};

const DESCRIPTION: &str = "Manipulate types.xml files for DayZ servers.";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = DESCRIPTION)]
struct Args {
    #[arg(index = 1)]
    file: String,
    #[command(subcommand)]
    action: Action,
}

#[derive(Clone, Debug, Subcommand)]
enum Action {
    #[command(long_about = "Merge an extension XML file into the base XML file")]
    Merge {
        #[arg(index = 1)]
        extension: String,
        #[arg(long, short)]
        output: Option<String>,
    },
    #[command(long_about = "Show the selected type's properties")]
    Show {
        #[arg(index = 1, name = "type")]
        name: String,
    },
    #[command(long_about = "Set the selected type's properties")]
    Set {
        #[arg(index = 1, name = "type")]
        name: String,
        #[command(subcommand)]
        field_value: FieldValue,
        #[arg(long, short)]
        output: Option<String>,
        #[arg(long, short)]
        in_place: bool,
    },
    #[command(long_about = "Add a new type")]
    Add {
        #[arg(index = 1, name = "type")]
        name: String,
        #[arg(long, short)]
        output: Option<String>,
        #[arg(long, short)]
        in_place: bool,
    },
    #[command(long_about = "Remove an existing type")]
    Remove {
        #[arg(index = 1, name = "type")]
        name: String,
        #[arg(long, short)]
        output: Option<String>,
        #[arg(long, short)]
        in_place: bool,
    },
}

#[derive(Clone, Debug, Subcommand)]
enum FieldValue {
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
enum FlagValues {
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

fn main() {
    let args = Args::parse();
    let mut types = read_types(&args.file);

    match args.action {
        Action::Merge { extension, output } => {
            write_type_or_exit(types + read_types(&extension), output.as_deref())
        }
        Action::Show { name } => {
            for typ in types.types().filter(|typ| {
                typ.get_name()
                    .to_ascii_lowercase()
                    .contains(&name.to_ascii_lowercase())
            }) {
                println!("{}", typ)
            }
        }
        Action::Set {
            name,
            field_value,
            output,
            in_place,
        } => {
            set_value(&mut types, &name, field_value);
            write_type_or_exit(
                types,
                if in_place {
                    Some(args.file.as_str())
                } else {
                    output.as_deref()
                },
            )
        }
        Action::Add {
            name,
            output,
            in_place,
        } => {
            types.add(Type::new(name));
            write_type_or_exit(
                types,
                if in_place {
                    Some(args.file.as_str())
                } else {
                    output.as_deref()
                },
            )
        }
        Action::Remove {
            name,
            output,
            in_place,
        } => {
            types.remove(&name);
            write_type_or_exit(
                types,
                if in_place {
                    Some(args.file.as_str())
                } else {
                    output.as_deref()
                },
            );
        }
    }
}

fn read_types(filename: &str) -> Types {
    Types::from_file(filename).unwrap_or_else(|error| {
        eprintln!("{}\n{}", filename, error);
        exit(1);
    })
}

fn set_value(types: &mut Types, name: &str, field_value: FieldValue) {
    if let Some(typ) = types
        .mut_types()
        .find(|typ| typ.get_name().to_ascii_lowercase() == name.to_ascii_lowercase())
    {
        match field_value {
            FieldValue::Name { name } => {
                typ.set_name(name);
            }
            FieldValue::Nominal { nominal } => {
                typ.set_nominal(nominal);
            }
            FieldValue::Lifetime { lifetime } => {
                typ.set_lifetime(lifetime);
            }
            FieldValue::Restock { restock } => {
                typ.set_restock(restock);
            }
            FieldValue::Min { min } => {
                typ.set_min(min);
            }
            FieldValue::Quantmin { quantmin } => {
                typ.set_quantmin(quantmin);
            }
            FieldValue::Quantmax { quantmax } => {
                typ.set_quantmax(quantmax);
            }
            FieldValue::Cost { cost } => {
                typ.set_cost(cost);
            }
            FieldValue::Flags { flags } => match flags {
                FlagValues::CountInCargo { count_in_cargo } => {
                    typ.mut_flags().set_count_in_cargo(count_in_cargo);
                }
                FlagValues::CountInHoarder { count_in_hoarder } => {
                    typ.mut_flags().set_count_in_hoarder(count_in_hoarder);
                }
                FlagValues::CountInMap { count_in_map } => {
                    typ.mut_flags().set_count_in_map(count_in_map);
                }
                FlagValues::CountInPlayer { count_in_player } => {
                    typ.mut_flags().set_count_in_player(count_in_player);
                }
                FlagValues::Crafted { crafted } => {
                    typ.mut_flags().set_crafted(crafted);
                }
                FlagValues::DeLoot { deloot } => {
                    typ.mut_flags().set_deloot(deloot);
                }
            },
            FieldValue::Category { category } => {
                typ.set_category(category);
            }
            FieldValue::Usages { usages } => {
                typ.set_usages(usages);
            }
            FieldValue::Values { values } => {
                typ.set_values(values);
            }
        }
    } else {
        eprintln!("No such type: {}", name);
        exit(4);
    }
}

fn write_type_or_exit(types: Types, filename: Option<&str>) {
    write_type(types, filename).unwrap_or_else(|error| {
        eprintln!("{}", error);
        exit(3);
    })
}

fn write_type(types: Types, filename: Option<&str>) -> Result<(), serde_rw::Error> {
    if let Some(filename) = filename {
        types.write_to_file(filename)
    } else {
        Ok(println!("{}", types.to_xml()?))
    }
}
