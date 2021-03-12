use std::str::FromStr;

use clap::Clap;

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

impl From<Command> for protocol::Command {
    fn from(command: Command) -> Self {
        match command {
            Command::Step(step) => Self::Step(step.into()),
        }
    }
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

impl From<Step> for protocol::Step {
    fn from(step: Step) -> Self {
        Self {
            direction: step.direction.into(),
            steps: step.steps,
            delay: step.delay,
        }
    }
}

#[derive(Clap, Debug)]
pub enum Direction {
    Forward,
    Backward,
}

impl From<Direction> for protocol::Direction {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Forward => Self::Forward,
            Direction::Backward => Self::Backward,
        }
    }
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
