use clap::{Parser, Subcommand};
use serde_rw::{FromFile, ToFile};
use std::process::exit;
use typesxml::{Named, Type, Types};

const DESCRIPTION: &str = "Merge types.xml files for DayZ servers.";

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
    },
    #[command(long_about = "Add a new type")]
    Add {
        #[arg(index = 1, name = "type")]
        name: String,
        #[arg(long, short)]
        output: Option<String>,
    },
    #[command(long_about = "Remove an existing type")]
    Remove {
        #[arg(index = 1, name = "type")]
        name: String,
        #[arg(long, short)]
        output: Option<String>,
    },
}

#[derive(Clone, Debug, Subcommand)]
enum FieldValue {
    Name {
        name: String,
    },
    Nominal {
        nominal: u8,
    },
    Lifetime {
        lifetime: u32,
    },
    Restock {
        restock: u32,
    },
    Min {
        min: u8,
    },
    Quantmin {
        quantmin: i64,
    },
    Quantmax {
        quantmax: i64,
    },
    Cost {
        cost: u32,
    },
    Flags {
        #[command(subcommand)]
        flags: FlagValues,
    },
    Category {
        category: Option<Named>,
    },
    Usages {
        usages: Option<Vec<Named>>,
    },
    Values {
        values: Option<Vec<Named>>,
    },
}

#[derive(Clone, Debug, Subcommand)]
enum FlagValues {
    CountInCargo { count_in_cargo: u64 },
    CountInHoarder { count_in_hoarder: u64 },
    CountInMap { count_in_map: u64 },
    CountInPlayer { count_in_player: u64 },
    Crafted { crafted: u64 },
    DeLoot { deloot: u64 },
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
                typ.name
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
        } => {
            set_value(&mut types, &name, field_value);
            write_type_or_exit(types, output.as_deref())
        }
        Action::Add { name, output } => {
            types.add(Type::new(name));
            write_type_or_exit(types, output.as_deref())
        }
        Action::Remove { name, output } => {
            types.remove(&name);
            write_type_or_exit(types, output.as_deref());
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
        .find(|typ| typ.name.to_ascii_lowercase() == name.to_ascii_lowercase())
    {
        match field_value {
            FieldValue::Name { name } => {
                typ.name = name;
            }
            FieldValue::Nominal { nominal } => {
                typ.nominal = nominal;
            }
            FieldValue::Lifetime { lifetime } => {
                typ.lifetime = lifetime;
            }
            FieldValue::Restock { restock } => {
                typ.restock = restock;
            }
            FieldValue::Min { min } => {
                typ.min = min;
            }
            FieldValue::Quantmin { quantmin } => {
                typ.quantmin = quantmin;
            }
            FieldValue::Quantmax { quantmax } => {
                typ.quantmax = quantmax;
            }
            FieldValue::Cost { cost } => {
                typ.cost = cost;
            }
            FieldValue::Flags { flags } => match flags {
                FlagValues::CountInCargo { count_in_cargo } => {
                    typ.flags.count_in_cargo = count_in_cargo;
                }
                FlagValues::CountInHoarder { count_in_hoarder } => {
                    typ.flags.count_in_hoarder = count_in_hoarder;
                }
                FlagValues::CountInMap { count_in_map } => {
                    typ.flags.count_in_map = count_in_map;
                }
                FlagValues::CountInPlayer { count_in_player } => {
                    typ.flags.count_in_player = count_in_player;
                }
                FlagValues::Crafted { crafted } => {
                    typ.flags.crafted = crafted;
                }
                FlagValues::DeLoot { deloot } => {
                    typ.flags.deloot = deloot;
                }
            },
            FieldValue::Category { category } => {
                typ.category = category;
            }
            FieldValue::Usages { usages } => {
                typ.usages = usages;
            }
            FieldValue::Values { values } => {
                typ.values = values;
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
