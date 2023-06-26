use crate::args::{read_types_or_exit, write_type_or_exit, Arguments};
use clap::Args;
use typesxml::Type;

#[derive(Clone, Debug, Args)]
pub struct Add {
    #[arg(index = 1, name = "type")]
    name: String,
    #[arg(long, short, help = "Write result to the given file instead of STDOUT")]
    output: Option<String>,
    #[arg(long, short, help = "Write result to the original file")]
    in_place: bool,
}

impl Add {
    pub fn run(&self, args: &Arguments) {
        let mut types = read_types_or_exit(args.file(), true);
        types.add(Type::new(&self.name));
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
