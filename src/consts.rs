use alloy_primitives::{I256, U256};

use crate::world::{Position, Velocity};

pub const WORLD_MAX_X: u32 = 10_000;
pub const WORLD_MAX_Y: u32 = 10_000;
pub const TICKS_PER_INPUT: u32 = 5;
pub const TICK_INPUT_API_CHUNK_SIZE: u32 = 10;
pub const VELOCITY_GAIN_NORMAL: &str = "5";
pub const VELOCITY_GAIN_BOOST: &str = "50";

pub const DEFAULT_RAFT_HEALTH: u32 = 10_000;

pub const DEFAULT_RAFT_WIDTH: u32 = WORLD_MAX_X / 4;
pub const DEFAULT_RAFT_HEIGHT: u32 = WORLD_MAX_Y / 10;

pub const DEFAULT_RAFT_FIGHTER_WIDTH: u32 = DEFAULT_RAFT_WIDTH / 10;
pub const DEFAULT_RAFT_FIGHTER_HEIGHT: u32 = DEFAULT_RAFT_HEIGHT / 10;


const DEFAULT_PROJECTILE_DIAMETER: u32 = WORLD_MAX_X / 50;
pub const DEFAULT_PROJECTILE_RADIUS: u32 = DEFAULT_PROJECTILE_DIAMETER / 2;

pub fn left_raft_init_pos() -> Position {
    Position {
        x: U256::from(WORLD_MAX_X / 10),
        y: U256::from(WORLD_MAX_Y / 4),
    }
}

pub fn right_raft_init_pos() -> Position {
    Position {
        x: U256::from(WORLD_MAX_X * 9 / 10 - DEFAULT_RAFT_WIDTH),
        y: U256::from(WORLD_MAX_Y / 4),
    }
}

pub fn no_velocity() -> Velocity {
    Velocity { vx: I256::ZERO, vy: I256::ZERO }
}

pub const LEFT_RAFT_MAX_X: u32 = WORLD_MAX_X / 2 - DEFAULT_RAFT_WIDTH;
pub const RIGHT_RAFT_MIN_X: u32 = WORLD_MAX_X / 2;

#[allow(clippy::cast_possible_wrap)]
const fn assert_u32_fits_in_i32(input: u32) -> i32 {
    assert!(input < i32::MAX as u32, "input too big");
    input as i32
}

pub const WORLD_MAX_X_I32: i32 = assert_u32_fits_in_i32(WORLD_MAX_X);
pub const WORLD_MAX_Y_I32: i32 = assert_u32_fits_in_i32(WORLD_MAX_Y);
