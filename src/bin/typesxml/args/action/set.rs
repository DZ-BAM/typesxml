use crate::args::FieldValue;
use crate::args::{read_types_or_exit, write_type_or_exit, Arguments};
use clap::Args;
use std::process::exit;
use typesxml::Types;

#[derive(Clone, Debug, Args)]
pub struct Set {
    #[arg(index = 1, name = "type")]
    name: String,
    #[command(subcommand)]
    field_value: FieldValue,
    #[arg(long, short, help = "Write result to the given file instead of STDOUT")]
    output: Option<String>,
    #[arg(long, short, help = "Write result to the original file")]
    in_place: bool,
}

impl Set {
    pub fn run(&self, args: &Arguments) {
        let mut types = read_types_or_exit(args.file(), true);
        self.set(&mut types);
        write_type_or_exit(
            &types,
            if self.in_place {
                Some(args.file())
            } else {
                self.output.as_deref()
            },
        );
    }

    fn set(&self, types: &mut Types) {
        types
            .mut_types()
            .find(|typ| typ.get_name().to_ascii_lowercase() == self.name.to_ascii_lowercase())
            .map_or_else(
                || {
                    eprintln!("No such type: {}", self.name);
                    exit(4);
                },
                |typ| self.field_value.set(typ),
            );
    }
}
