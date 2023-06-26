mod args;

use args::Arguments;
use clap::Parser;

fn main() {
    Arguments::parse().run();
}
