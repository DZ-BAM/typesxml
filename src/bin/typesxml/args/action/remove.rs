use crate::args::{read_types_or_exit, write_type_or_exit, Arguments, Run};
use clap::Args;

#[derive(Clone, Debug, Args)]
pub struct Remove {
    #[arg(index = 1, name = "type")]
    name: String,
    #[arg(long, short)]
    output: Option<String>,
    #[arg(long, short)]
    in_place: bool,
}

impl Run for Remove {
    fn run(&self, args: &Arguments) {
        let mut types = read_types_or_exit(args.file(), true);
        types.remove(&self.name);
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
