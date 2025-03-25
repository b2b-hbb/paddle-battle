use std::cmp::Ordering;

use alloy_primitives::{U256, I256};

use crate::world::{Projectile, Raft, RaftFighter};

pub trait SaturatingAddSigned {
    fn saturating_add_signed(&self, other: I256) -> Self;
}

impl SaturatingAddSigned for U256 {
    fn saturating_add_signed(&self, other: I256) -> Self {
        if other.is_negative() {
            // For negative values, convert to positive and saturating subtract
            let abs = other.abs();
            // If abs() would overflow U256, return 0 since we're subtracting more than our value
            if abs > I256::from_raw(U256::MAX) {
                U256::ZERO
            } else {
                let abs_bytes: [u8; 32] = abs.to_be_bytes();
                let abs_u256 = U256::from_be_bytes(abs_bytes);
                self.saturating_sub(abs_u256)
            }
        } else {
            // For positive values, saturating add
            // If other would overflow U256, return U256::MAX
            if other > I256::from_raw(U256::MAX) {
                U256::MAX
            } else {
                let other_bytes: [u8; 32] = other.to_be_bytes();
                let other_u256 = U256::from_be_bytes(other_bytes);
                self.saturating_add(other_u256)
            }
        }
    }
}

pub trait Collision {
    /// Checks if the current object collides with another object of the same trait.
    fn collides_with<T: Collision>(&self, other: &T) -> bool;

    /// Returns the bounding box of the object as (x, y, width, height).
    fn bounding_box(&self) -> (U256, U256, U256, U256);
}

impl Collision for Raft {
    fn collides_with<T: Collision>(&self, other: &T) -> bool {
        let (x1, y1, w1, h1) = self.bounding_box();
        let (x2, y2, w2, h2) = other.bounding_box();

        // AABB collision detection
        x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
    }

    fn bounding_box(&self) -> (U256, U256, U256, U256) {
        (
            self.entity.position.x,
            self.entity.position.y,
            self.width,
            self.height,
        )
    }
}

// TODO: DRY up raftfighter and raft code in collides with and
impl Collision for RaftFighter {
    fn collides_with<T: Collision>(&self, other: &T) -> bool {
        let (x1, y1, w1, h1) = self.bounding_box();
        let (x2, y2, w2, h2) = other.bounding_box();

        // AABB collision detection
        x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
    }

    fn bounding_box(&self) -> (U256, U256, U256, U256) {
        (
            self.entity.position.x,
            self.entity.position.y,
            self.width,
            self.height,
        )
    }
}

impl Collision for Projectile {
    fn collides_with<T: Collision>(&self, other: &T) -> bool {
        let (x1, y1, w1, h1) = self.bounding_box();
        let (x2, y2, w2, h2) = other.bounding_box();

        // AABB collision detection, simpler but less accurate for circles
        x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
        // TODO: check actual boundaries after cheaper check above
    }

    fn bounding_box(&self) -> (U256, U256, U256, U256) {
        let diameter = self.radius * U256::from(2);
        // (x1, y1, w1, h1)
        (
            self.entity.position.x.saturating_sub(self.radius),
            self.entity.position.y.saturating_sub(self.radius),
            // width smaller on purpose so projectile goes into other entity
            self.radius,
            diameter,
        )
    }
}

impl Raft {
    pub fn update_position(&mut self, curr_tick: U256) {
        // apply velocity
        self.entity.position.x = self
            .entity
            .position
            .x
            .saturating_add_signed(self.entity.velocity.vx);
        self.entity.position.y = self
            .entity
            .position
            .y
            .saturating_add_signed(self.entity.velocity.vy);

        for fighter in &mut *self.raft_fighters {
            fighter.entity.position.x = fighter
                .entity
                .position
                .x
                .saturating_add_signed(self.entity.velocity.vx);
            fighter.entity.position.y = fighter
                .entity
                .position
                .y
                .saturating_add_signed(self.entity.velocity.vy);
        }

        // apply velocity decay
        if (curr_tick % U256::from(20)).is_zero() {
            self.entity.velocity.vx = self.entity.velocity.vx / I256::from_dec_str("2").unwrap();
            self.entity.velocity.vy = self.entity.velocity.vy / I256::from_dec_str("2").unwrap();
        }
    }
}

impl Projectile {
    pub fn update_position(&mut self, curr_tick: U256) {
        // apply velocity
        self.entity.position.x = self
            .entity
            .position
            .x
            .saturating_add_signed(self.entity.velocity.vx);
        // apply sine wave to y position using lookup table
        let AMPLITUDE: U256 = U256::from(100);
        let FREQUENCY: U256 = U256::from(1);
        // Precomputed sine wave values scaled by 1000 to avoid floating points
        let SINE_TABLE: [I256; 8] = [
            I256::from_dec_str("0").unwrap(),
            I256::from_dec_str("707").unwrap(),
            I256::from_dec_str("1000").unwrap(),
            I256::from_dec_str("707").unwrap(),
            I256::from_dec_str("0").unwrap(),
            I256::from_dec_str("-707").unwrap(),
            I256::from_dec_str("-1000").unwrap(),
            I256::from_dec_str("-707").unwrap()
        ];
        
        // Convert the table length to U256 for modulo operation
        let table_len = U256::from(SINE_TABLE.len());
        let index = (curr_tick * FREQUENCY) % table_len;
        // Convert back to usize for array indexing
        let array_index = index.to::<usize>();
        let sine_value = SINE_TABLE[array_index];
        
        // Calculate y_offset using the sine value
        let y_offset = (sine_value * I256::from_raw(AMPLITUDE)) / I256::from_dec_str("1000").unwrap();
        self.entity.position.y = self.entity.position.y.saturating_add_signed(y_offset);

        // let frequency = 0.1; // Adjust the frequency of the sine wave
        // self.entity.position.y = self.entity.position.y + (amplitude as f32 * (curr_tick as f32 * frequency).sin()) as u32;

        // apply velocity decay
        let decay_interval = U256::from(50);
        if (curr_tick % decay_interval).is_zero() {
            match self.entity.velocity.vx.cmp(&I256::ZERO) {
                Ordering::Less => self.entity.velocity.vx = self.entity.velocity.vx + I256::from_dec_str("1").unwrap(),
                Ordering::Greater => self.entity.velocity.vx = self.entity.velocity.vx - I256::from_dec_str("1").unwrap(),
                Ordering::Equal => (),
            };
        }
    }
}
