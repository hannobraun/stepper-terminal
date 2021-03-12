#![no_std]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Command {
    Step(Step),
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
