use crate::args::FieldValue;
use crate::args::{read_types_or_exit, set_value, write_type_or_exit, Arguments, Run};
use clap::Args;

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

impl Run for Set {
    fn run(&self, args: &Arguments) {
        let mut types = read_types_or_exit(args.file(), true);
        set_value(&mut types, &self.name, &self.field_value);
        write_type_or_exit(
            &types,
            if self.in_place {
                Some(args.file())
            } else {
                self.output.as_deref()
            },
        );
    }
}
