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
    let merger = Types::from_str(&read_to_string(args.base).unwrap_or_else(|error| {
        eprintln!("{}", error);
        exit(1);
    }))
    .unwrap_or_else(|error| {
        eprintln!("{}", error);
        exit(2);
    }) + Types::from_str(&read_to_string(args.extension).unwrap_or_else(|error| {
        eprintln!("{}", error);
        exit(1);
    }))
    .unwrap_or_else(|error| {
        eprintln!("{}", error);
        exit(2);
    });

    if let Some(file) = args.output {
        write(file, merger.to_string()).unwrap_or_else(|error| {
            eprintln!("{}", error);
            exit(3);
        })
    } else {
        println!("{}", merger);
    }
}
