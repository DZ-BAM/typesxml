use crate::args::{Action, Run};
use clap::Parser;

const DESCRIPTION: &str = "Manipulate types.xml files for DayZ servers.";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = DESCRIPTION)]
pub struct Arguments {
    #[arg(index = 1)]
    file: String,
    #[command(subcommand)]
    action: Action,
}

impl Arguments {
    pub fn run(&self) {
        self.action.run(self);
    }

    pub fn file(&self) -> &str {
        self.file.as_str()
    }
}
