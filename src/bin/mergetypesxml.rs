use clap::Parser;
use from_file::FromFile;
use std::fs::write;
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
    );
}

fn read(filename: &str) -> Types {
    Types::from_file(filename).unwrap_or_else(|error| {
        eprintln!("{}\n{}", filename, error);
        exit(1);
    })
}

fn output(types: Types, filename: Option<&str>) {
    if let Some(filename) = filename {
        write(filename, types.to_string()).unwrap_or_else(|error| {
            eprintln!("{}", error);
            exit(3);
        })
    } else {
        println!("{}", types);
    }
}
