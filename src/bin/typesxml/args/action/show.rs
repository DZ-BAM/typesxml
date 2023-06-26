use crate::args::{read_types_or_exit, Arguments, Run};
use clap::Args;
use serde_rw::ToXml;

#[derive(Clone, Debug, Args)]
pub struct Show {
    #[arg(index = 1, name = "type")]
    name: String,
    #[arg(long, short, help = "Show type as XML")]
    xml: bool,
}

impl Run for Show {
    fn run(&self, args: &Arguments) {
        read_types_or_exit(args.file(), true)
            .types()
            .find(|typ| typ.get_name().to_ascii_lowercase() == self.name.to_ascii_lowercase())
            .map_or_else(
                || {
                    eprintln!("No such type: {}", self.name);
                },
                |typ| {
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
                },
            );
    }
}
