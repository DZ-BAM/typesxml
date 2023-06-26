use crate::args::{read_types_or_exit, Arguments, Run};
use clap::Args;
use regex::Regex;
use serde_rw::ToXml;

#[derive(Clone, Debug, Args)]
pub struct Find {
    #[arg(index = 1, name = "type")]
    regex: Regex,
    #[arg(long, short, help = "Show type as XML")]
    xml: bool,
}

impl Run for Find {
    fn run(&self, args: &Arguments) {
        for typ in read_types_or_exit(args.file(), true)
            .types()
            .filter(|typ| self.regex.is_match(typ.get_name()))
        {
            if self.xml {
                typ.to_xml_pretty(' ', 4).map_or_else(
                    |_| eprintln!("Could not serialize XML"),
                    |xml| {
                        println!("{xml}");
                    },
                );
            } else {
                println!("{typ}");
            }
        }
    }
}
