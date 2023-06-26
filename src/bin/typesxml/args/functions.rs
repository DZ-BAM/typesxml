use crate::args::{FieldValue, FlagValues};
use serde_rw::{FromFile, ToFile};
use std::process::exit;
use typesxml::Types;

pub fn read_types_or_exit(filename: &str, strict: bool) -> Types {
    if strict {
        Types::from_file(filename)
    } else {
        Types::read_gracefully(filename)
    }
    .unwrap_or_else(|error| {
        eprintln!("{filename}\n{error}");
        exit(1);
    })
}

pub fn write_type_or_exit(types: &Types, filename: Option<&str>) {
    filename.map_or_else(
        || println!("{types}"),
        |filename| {
            types
                .write_to_file_pretty(filename)
                .unwrap_or_else(|error| {
                    eprintln!("{error}");
                    exit(3);
                });
        },
    );
}

pub fn set_value(types: &mut Types, name: &str, field_value: &FieldValue) {
    types
        .mut_types()
        .find(|typ| typ.get_name().to_ascii_lowercase() == name.to_ascii_lowercase())
        .map_or_else(
            || {
                eprintln!("No such type: {name}");
                exit(4);
            },
            |typ| match field_value {
                FieldValue::Name { name } => {
                    typ.set_name(name);
                }
                FieldValue::Nominal { nominal } => {
                    typ.set_nominal(*nominal);
                }
                FieldValue::Lifetime { lifetime } => {
                    typ.set_lifetime(*lifetime);
                }
                FieldValue::Restock { restock } => {
                    typ.set_restock(*restock);
                }
                FieldValue::Min { min } => {
                    typ.set_min(*min);
                }
                FieldValue::Quantmin { quantmin } => {
                    typ.set_quantmin(*quantmin);
                }
                FieldValue::Quantmax { quantmax } => {
                    typ.set_quantmax(*quantmax);
                }
                FieldValue::Cost { cost } => {
                    typ.set_cost(*cost);
                }
                FieldValue::Flags { flags } => match flags {
                    FlagValues::CountInCargo { count_in_cargo } => {
                        typ.mut_flags().set_count_in_cargo(*count_in_cargo);
                    }
                    FlagValues::CountInHoarder { count_in_hoarder } => {
                        typ.mut_flags().set_count_in_hoarder(*count_in_hoarder);
                    }
                    FlagValues::CountInMap { count_in_map } => {
                        typ.mut_flags().set_count_in_map(*count_in_map);
                    }
                    FlagValues::CountInPlayer { count_in_player } => {
                        typ.mut_flags().set_count_in_player(*count_in_player);
                    }
                    FlagValues::Crafted { crafted } => {
                        typ.mut_flags().set_crafted(*crafted);
                    }
                    FlagValues::DeLoot { deloot } => {
                        typ.mut_flags().set_deloot(*deloot);
                    }
                },
                FieldValue::Category { category } => {
                    typ.set_category(category.as_ref());
                }
                FieldValue::Usages { usages } => {
                    typ.set_usages(usages.as_deref());
                }
                FieldValue::Values { values } => {
                    typ.set_values(values.as_deref());
                }
            },
        );
}
