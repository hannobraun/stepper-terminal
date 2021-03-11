use std::str::FromStr;

use clap::Clap;

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}

#[derive(Clap, Debug)]
#[clap(name = "Stepper Terminal")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Clap, Debug)]
pub enum Command {
    /// Step the stepper motor
    Step(Step),
}

#[derive(Clap, Debug)]
pub struct Step {
    /// The direction to move into ("forward"/"backward")
    #[clap(short, long, default_value = "forward")]
    pub direction: Direction,

    /// The number of steps to take
    #[clap(short, long, default_value = "1")]
    pub steps: u32,

    /// The delay between steps in milliseconds
    #[clap(long, default_value = "100")]
    pub delay: u32,
}

#[derive(Clap, Debug)]
pub enum Direction {
    Forward,
    Backward,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "backward" => Ok(Self::Backward),
            invalid => Err(invalid.to_owned()),
        }
    }
}
