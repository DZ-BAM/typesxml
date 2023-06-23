mod args;

use crate::args::{Action, FieldValue, FlagValues};
use args::CLIArgs;
use clap::Parser;
use serde_rw::{FromFile, ToFile};
use std::process::exit;
use typesxml::{Type, Types};

fn main() {
    let args = CLIArgs::parse();

    match args.action {
        Action::Add(add) => {
            let mut types = read_types_or_exit(&args.file, true);
            types.add(Type::new(add.name));
            write_type_or_exit(
                types,
                if add.in_place {
                    Some(args.file.as_str())
                } else {
                    add.output.as_deref()
                },
            )
        }
        Action::Find(find) => {
            for typ in read_types_or_exit(&args.file, true)
                .types()
                .filter(|typ| find.regex.is_match(typ.get_name()))
            {
                println!("{}", typ)
            }
        }
        Action::Fix => write_type_or_exit(read_types_or_exit(&args.file, false), Some(&args.file)),
        Action::Merge(merge) => write_type_or_exit(
            read_types_or_exit(&args.file, true) + read_types_or_exit(&merge.extension, true),
            merge.output.as_deref(),
        ),
        Action::Remove(remove) => {
            let mut types = read_types_or_exit(&args.file, true);
            types.remove(&remove.name);
            write_type_or_exit(
                types,
                if remove.in_place {
                    Some(args.file.as_str())
                } else {
                    remove.output.as_deref()
                },
            );
        }
        Action::Set(set) => {
            let mut types = read_types_or_exit(&args.file, true);
            set_value(&mut types, &set.name, set.field_value);
            write_type_or_exit(
                types,
                if set.in_place {
                    Some(args.file.as_str())
                } else {
                    set.output.as_deref()
                },
            )
        }
    }
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

fn read_types_or_exit(filename: &str, strict: bool) -> Types {
    if strict {
        Types::from_file(filename)
    } else {
        Types::read_gracefully(filename)
    }
    .unwrap_or_else(|error| {
        eprintln!("{}\n{}", filename, error);
        exit(1);
    })
}

fn write_type_or_exit(types: Types, filename: Option<&str>) {
    if let Some(filename) = filename {
        types.write_to_file(filename)
    } else {
        types.to_xml().map(|types| println!("{}", types))
    }
    .unwrap_or_else(|error| {
        eprintln!("{}", error);
        exit(3);
    })
}
