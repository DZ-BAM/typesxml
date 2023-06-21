use clap::Parser;
use std::fs::{read_to_string, write};
use std::process::exit;
use std::str::FromStr;
use typesxml::Types;

const DESCRIPTION: &str = "Merge types.xml files for DayZ servers.";

#[derive(Parser, Debug)]
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
    let merger = types_from_file(&args.base) + types_from_file(&args.extension);

    if let Some(file) = args.output {
        write(file, merger.to_string()).unwrap_or_else(|error| {
            eprintln!("{}", error);
            exit(3);
        })
    } else {
        println!("{}", merger);
    }
}

fn types_from_file(filename: &str) -> Types {
    Types::from_str(&read_to_string(filename).unwrap_or_else(|error| {
        eprintln!("{}\n{}", filename, error);
        exit(1);
    }))
    .unwrap_or_else(|error| {
        eprintln!("{} in {}", error, filename);
        exit(2);
    })
}
