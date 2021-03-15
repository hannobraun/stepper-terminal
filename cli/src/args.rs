use clap::Clap;
use protocol::Direction;

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
    /// The number of steps to take (negative value means backward movement)
    #[clap(short, long, default_value = "1")]
    pub steps: i32,

    /// The delay between steps in milliseconds
    #[clap(long, default_value = "10")]
    pub delay: u32,
}

impl From<Step> for protocol::Step {
    fn from(step: Step) -> Self {
        let direction = if step.steps >= 0 {
            Direction::Forward
        } else {
            Direction::Backward
        };

        Self {
            direction,
            steps: step.steps.abs() as u32,
            delay: step.delay,
        }
    }
}
