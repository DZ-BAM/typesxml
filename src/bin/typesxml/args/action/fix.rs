use crate::args::{read_types_or_exit, write_type_or_exit, Arguments, Run};
use clap::Args;

#[derive(Clone, Debug, Args)]
pub struct Fix;

impl Run for Fix {
    fn run(&self, args: &Arguments) {
        write_type_or_exit(&read_types_or_exit(args.file(), false), Some(args.file()));
    }
}
