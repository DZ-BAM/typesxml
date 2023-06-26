use crate::args::{Arguments, Run};
use clap::Subcommand;

mod add;
mod find;
mod fix;
mod merge;
mod remove;
mod set;
mod show;

#[derive(Clone, Debug, Subcommand)]
pub enum Action {
    #[command(long_about = "Add a new type")]
    Add(add::Add),
    #[command(long_about = "Display the selected type's properties")]
    Find(find::Find),
    #[command(long_about = "Fix errors in the given file")]
    Fix(fix::Fix),
    #[command(long_about = "Merge an extension XML file into the base XML file")]
    Merge(merge::Merge),
    #[command(long_about = "Remove an existing type")]
    Remove(remove::Remove),
    #[command(long_about = "Set the selected type's properties")]
    Set(set::Set),
    #[command(long_about = "Show the selected type")]
    Show(show::Show),
}

impl Run for Action {
    fn run(&self, args: &Arguments) {
        match self {
            Self::Add(action) => action.run(args),
            Self::Find(action) => action.run(args),
            Self::Fix(action) => action.run(args),
            Self::Merge(action) => action.run(args),
            Self::Remove(action) => action.run(args),
            Self::Set(action) => action.run(args),
            Self::Show(action) => action.run(args),
        }
    }
}
