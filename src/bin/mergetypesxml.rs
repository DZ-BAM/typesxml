use clap::Parser;
use serde_rw::{FromFile, ToFile};
use std::process::exit;
use typesxml::Types;

const DESCRIPTION: &str = "Merge types.xml files for DayZ servers.";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = DESCRIPTION)]
struct Args {
    #[arg(index = 1)]
    base: String,

    #[arg(index = 2)]
    extension: String,

    #[arg(long, short)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    output(
        read(&args.base) + read(&args.extension),
        args.output.as_deref(),
    )
    .unwrap_or_else(|error| {
        eprintln!("{}", error);
        exit(3);
    });
}

fn read(filename: &str) -> Types {
    Types::from_file(filename).unwrap_or_else(|error| {
        eprintln!("{}\n{}", filename, error);
        exit(1);
    })
}

fn output(types: Types, filename: Option<&str>) -> Result<(), serde_rw::Error> {
    if let Some(filename) = filename {
        types.write_to_file(filename)
    } else {
        Ok(println!("{}", types.to_xml()?))
    }
}
