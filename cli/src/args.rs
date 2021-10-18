use clap::Clap;
use protocol::Direction;

#[derive(Clap, Debug)]
#[clap(name = "Stepper Terminal")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

impl Args {
    pub fn parse() -> Self {
        <Self as Clap>::parse()
    }
}

#[derive(Clap, Debug)]
pub enum Command {
    /// Step the stepper motor
    Step(Step),

    /// Move to a position, while respecting a maximum speed
    MoveTo(MoveTo),
}

impl From<Command> for protocol::Command {
    fn from(command: Command) -> Self {
        match command {
            Command::Step(step) => Self::Step(step.into()),
            Command::MoveTo(move_to) => Self::MoveTo(move_to.into()),
        }
    }
}

#[derive(Clap, Debug)]
pub struct Step {
    /// The number of steps to take (negative value means backward movement)
    #[clap(short, long, default_value = "200")]
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

#[derive(Clap, Debug)]
pub struct MoveTo {
    /// The target step (absolute position)
    pub target_step: i32,

    /// The maximum speed in steps per second
    #[clap(short, long, default_value = "1000")]
    pub max_speed: u16,
}

impl From<MoveTo> for protocol::MoveTo {
    fn from(move_to: MoveTo) -> Self {
        Self {
            target_step: move_to.target_step,
            max_speed: move_to.max_speed,
        }
    }
}
