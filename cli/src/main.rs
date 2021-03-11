mod args;

use clap::Clap as _;

use self::args::Args;

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
