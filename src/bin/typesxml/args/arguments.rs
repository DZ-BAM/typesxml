use crate::args::Action;
use clap::Parser;

const DESCRIPTION: &str = "Manipulate types.xml files for DayZ servers.";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = DESCRIPTION, subcommand_value_name = "action", subcommand_help_heading = "Actions")]
pub struct Arguments {
    #[arg(index = 1, name = "xml_file", help = "The base XML file to operate on")]
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
