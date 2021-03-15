#![no_std]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Command {
    Step(Step),
    MoveTo(MoveTo),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Step {
    pub direction: Direction,
    pub steps: u32,
    pub delay: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MoveTo {
    pub target_step: i32,
    pub max_speed: u16,
}
