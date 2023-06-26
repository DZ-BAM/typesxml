use crate::args::Arguments;

pub trait Run {
    fn run(&self, args: &Arguments);
}
