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
