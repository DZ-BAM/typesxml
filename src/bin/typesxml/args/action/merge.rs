use crate::args::{read_types_or_exit, write_type_or_exit, Arguments};
use clap::Args;

#[derive(Clone, Debug, Args)]
pub struct Merge {
    #[arg(index = 1)]
    extension: String,
    #[arg(long, short, help = "Write result to the given file instead of STDOUT")]
    output: Option<String>,
}

impl Merge {
    pub fn run(&self, args: &Arguments) {
        write_type_or_exit(
            &(read_types_or_exit(args.file(), true) + read_types_or_exit(&self.extension, true)),
            self.output.as_deref(),
        );
    }
}
