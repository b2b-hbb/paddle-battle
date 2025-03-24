use std::cmp::Ordering;

use crate::world::{Projectile, Raft, RaftFighter};

pub trait Collision {
    /// Checks if the current object collides with another object of the same trait.
    fn collides_with<T: Collision>(&self, other: &T) -> bool;

    /// Returns the bounding box of the object as (x, y, width, height).
    fn bounding_box(&self) -> (u32, u32, u32, u32);
}

impl Collision for Raft {
    fn collides_with<T: Collision>(&self, other: &T) -> bool {
        let (x1, y1, w1, h1) = self.bounding_box();
        let (x2, y2, w2, h2) = other.bounding_box();

        // AABB collision detection
        x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
    }

    fn bounding_box(&self) -> (u32, u32, u32, u32) {
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

    fn bounding_box(&self) -> (u32, u32, u32, u32) {
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

    fn bounding_box(&self) -> (u32, u32, u32, u32) {
        let diameter = 2 * self.radius;
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
    pub fn update_position(&mut self, curr_tick: u32) {
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
        if curr_tick % 20 == 0 {
            self.entity.velocity.vx /= 2;
            self.entity.velocity.vy /= 2;
        }
    }
}

impl Projectile {
    pub fn update_position(&mut self, curr_tick: u32) {
        // apply velocity
        self.entity.position.x = self
            .entity
            .position
            .x
            .saturating_add_signed(self.entity.velocity.vx);

        // apply sine wave to y position
        let amplitude = 10; // Adjust the amplitude of the sine wave
        let frequency = 0.1; // Adjust the frequency of the sine wave
        self.entity.position.y = self.entity.position.y + (amplitude as f32 * (curr_tick as f32 * frequency).sin()) as u32;

        // apply velocity decay
        if curr_tick % 50 == 0 {
            match self.entity.velocity.vx.cmp(&0) {
                Ordering::Less => self.entity.velocity.vx += 1,
                Ordering::Greater => self.entity.velocity.vx -= 1,
                Ordering::Equal => (),
            };
        }
    }
}
