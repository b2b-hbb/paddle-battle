extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use alloc::vec;

use crate::consts;

use crate::errors::Result;
use crate::errors::SimulationError;
use crate::physics::Collision;
use crate::world::Bearings;
use crate::world::GunTypes;
use crate::world::RaftFighter;
use crate::world::Style;
use crate::world::{Entity, GameState, Position, Projectile, Raft, Velocity};

#[cfg(test)]
use strum_macros::{EnumCount, EnumIter};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq, EnumIter, EnumCount))]
pub enum GameInput {
    NoOp,
    DeprecatedShootLeftRaft,
    MoveLeftRaftRight,
    MoveLeftRaftLeft,
    DeprecatedShootRightRaft,
    MoveRightRaftRight,
    MoveRightRaftLeft,
    DeprecatedAddProjectile,
    MoveLeftFastRaftLeft,
    MoveLeftFastRaftRight,
    MoveUpRaftRight,
    MoveRightRaftDown,
    MoveUpRaftLeft,
    MoveLeftRaftDown,
}

impl GameInput {
    /// # Errors
    ///
    /// Will return `Err` if `input` does not exist as a valid game input
    pub const fn from(input: u32) -> Result<Self> {
        match input {
            0 => Ok(Self::DeprecatedShootLeftRaft),
            1 => Ok(Self::MoveLeftRaftRight),
            2 => Ok(Self::MoveLeftRaftLeft),
            3 => Ok(Self::DeprecatedShootRightRaft),
            4 => Ok(Self::MoveRightRaftRight),
            5 => Ok(Self::MoveRightRaftLeft),
            6 => Ok(Self::DeprecatedAddProjectile),
            7 => Ok(Self::MoveLeftFastRaftLeft),
            8 => Ok(Self::MoveLeftFastRaftRight),
            9 => Ok(Self::MoveUpRaftRight),
            10 => Ok(Self::MoveRightRaftDown),
            11 => Ok(Self::MoveUpRaftLeft),
            12 => Ok(Self::MoveLeftRaftDown),
            86 => Ok(Self::NoOp),
            received => Err(SimulationError::InvalidInput { received }),
        }
    }

    #[must_use]
    pub const fn to_u32(&self) -> u32 {
        match self {
            Self::DeprecatedShootLeftRaft => 0,
            Self::MoveLeftRaftRight => 1,
            Self::MoveLeftRaftLeft => 2,
            Self::DeprecatedShootRightRaft => 3,
            Self::MoveRightRaftRight => 4,
            Self::MoveRightRaftLeft => 5,
            Self::DeprecatedAddProjectile => 6,
            Self::MoveLeftFastRaftLeft => 7,
            Self::MoveLeftFastRaftRight => 8,
            Self::MoveUpRaftRight => 9,
            Self::MoveRightRaftDown => 10,
            Self::MoveUpRaftLeft => 11,
            Self::MoveLeftRaftDown => 12,
            Self::NoOp => 86,
        }
    }
}

impl GameState {
    #[must_use]
    pub fn new() -> Self {
        let mut raft_left = Raft::new(
            Entity {
                position: consts::LEFT_RAFT_INIT_POS,
                velocity: consts::NO_VELOCITY,
                is_active: true,
            },
            Style {
                color: String::from("#FF0000"),
            },
        );

        let mut raft_right = Raft::new(
            Entity {
                position: Position {
                    x: consts::WORLD_MAX_X - consts::DEFAULT_RAFT_WIDTH,
                    y: consts::RIGHT_RAFT_INIT_POS.y,
                },
                velocity: consts::NO_VELOCITY,
                is_active: true,
            },
            Style {
                color: String::from("#0000FF"),
            },
        );

        let left_fighter1 = RaftFighter::new(
            Entity {
                position: Position {
                    x: raft_left.entity.position.x + raft_left.width * 4 / 5,
                    y: raft_left.entity.position.y + raft_left.height * 4 / 5,
                },
                velocity: consts::NO_VELOCITY,
                is_active: true,
            },
            GunTypes::SMG,
            consts::DEFAULT_RAFT_FIGHTER_WIDTH,
            consts::DEFAULT_RAFT_FIGHTER_HEIGHT,
        );

        let right_fighter1 = RaftFighter::new(
            Entity {
                position: Position {
                    x: raft_right.entity.position.x + raft_right.width * 1 / 5,
                    y: raft_right.entity.position.y + raft_right.height * 4 / 5,
                },
                velocity: consts::NO_VELOCITY,
                is_active: true,
            },
            GunTypes::Bazooka,
            consts::DEFAULT_RAFT_FIGHTER_WIDTH,
            consts::DEFAULT_RAFT_FIGHTER_HEIGHT,
        );

        let right_fighter2 = RaftFighter::new(
            Entity {
                position: Position {
                    x: raft_right.entity.position.x + raft_right.width * 2 / 5,
                    y: raft_right.entity.position.y + raft_right.height * 4 / 5,
                },
                velocity: consts::NO_VELOCITY,
                is_active: true,
            },
            GunTypes::SMG,
            consts::DEFAULT_RAFT_FIGHTER_WIDTH,
            consts::DEFAULT_RAFT_FIGHTER_HEIGHT,
        );

        raft_left.position_fighters(vec![left_fighter1]);
        raft_right.position_fighters(vec![right_fighter1, right_fighter2]);

        Self {
            raft_left,
            raft_right,
            left_projectiles: vec![],
            right_projectiles: vec![],
            ticks: 0,
        }
    }

    /// # Errors
    ///
    /// Will return Err if input is invalid
    #[allow(clippy::too_many_lines)]
    pub fn tick(&mut self, ticks_to_process: u32, input: &Vec<u32>) -> Result<()> {
        let initial_tick = self.ticks;
        let end_tick = initial_tick + ticks_to_process;
        let inputs_needed = tick_inputs_needed(ticks_to_process);
        let Some(input_len) = u32::try_from(input.len()).ok() else {
            return Err(SimulationError::USizeToU32Conversion {});
        };

        if input_len != inputs_needed {
            return Err(SimulationError::InvalidInputLength {
                received: input_len,
                expected: inputs_needed,
                initial_tick,
                end_tick,
            });
        }

        // TODO: does chunk and iterator result in same performance than manually calculating index?
        // let input_index = (tick / consts::TICKS_PER_INPUT) as usize - current_input_index;
        let chunked_input = input
            .chunks(consts::TICK_INPUT_API_CHUNK_SIZE as usize)
            .map(<[u32]>::to_vec)
            .collect::<Vec<Vec<u32>>>();
        let mut iter = chunked_input.iter();

        for curr_tick in initial_tick..end_tick {
            let raft_left = &mut self.raft_left;
            let raft_right = &mut self.raft_right;

            /*
             * here we first move entities then attempt to detect collision
             * this assumes that the initial state does not include collisions (which could potentially lead to invalid states)
             * it also assumes no sideeffects on positions are created before this codepath
             * ie the input handling above never directly alters an entity's position, but instead just mutates its velocity
             * which is then applied below
             */

            /*
             * Ideal pattern here would be a "stage then commit"
             *
             * let temp = projectile.clone();
             * update_position(temp, curr_tick);
             * if valid(temp) { *projectile = temp; }
             *
             * but its too expensive to clone all projectiles to extend this pattern
             * so you can roll back changes in case its needed
             *
             * With rafts we do a clone of the entity to increase simplicity since there are only 2
             * but you could also instead do it inline with approach commented out below to avoid clones
             *
             * let (x, y, width, height) = raft_left.bounding_box();
             * raft_left.entity.position.x = x.min(consts::WORLD_MAX_X - width);
             * raft_left.entity.position.y = y.min(consts::WORLD_MAX_Y - height);
             *
             * these are also slightly different approaches since now the raft isnt set to
             * the max possible position and will instead remain in its prev valid position
             */

            if curr_tick % consts::TICKS_PER_INPUT == 0 {
                handle_input(iter.next(), raft_left, raft_right, curr_tick)?;
            }

            update_raft(raft_left, curr_tick);
            update_raft(raft_right, curr_tick);

            update_fighters(
                raft_left,
                &mut self.left_projectiles,
                Bearings::Northeast,
                curr_tick,
            );
            update_fighters(
                raft_right,
                &mut self.right_projectiles,
                Bearings::Northwest,
                curr_tick,
            );

            update_projectiles(&mut self.left_projectiles, raft_right, curr_tick);
            update_projectiles(&mut self.right_projectiles, raft_left, curr_tick);

            self.ticks += 1;
        }
        Ok(())
    }
}

fn handle_input(
    input_for_tick: Option<&Vec<u32>>,
    raft_left: &mut Raft,
    raft_right: &mut Raft,
    curr_tick: u32,
) -> Result<()> {
    let input_for_tick = input_for_tick.ok_or(SimulationError::NoInput {
        num_tick: curr_tick,
    })?;

    for &curr in input_for_tick {
        match GameInput::from(curr)? {
            GameInput::NoOp => {}
            GameInput::DeprecatedShootLeftRaft => {}
            GameInput::DeprecatedShootRightRaft => {}
            GameInput::MoveLeftRaftRight => {
                raft_left.entity.velocity.vx = consts::VELOCITY_GAIN_NORMAL;
            }
            GameInput::MoveLeftRaftLeft => {
                raft_left.entity.velocity.vx = -consts::VELOCITY_GAIN_NORMAL;
            }
            GameInput::MoveRightRaftRight => {
                raft_right.entity.velocity.vx = consts::VELOCITY_GAIN_NORMAL;
            }
            GameInput::MoveUpRaftRight => {
                raft_right.entity.velocity.vy = consts::VELOCITY_GAIN_NORMAL;
            }
            GameInput::MoveRightRaftLeft => {
                raft_right.entity.velocity.vx = -consts::VELOCITY_GAIN_NORMAL;
            }
            GameInput::MoveLeftFastRaftLeft => {
                raft_left.entity.velocity.vx = consts::VELOCITY_GAIN_BOOST;
            }
            GameInput::MoveLeftFastRaftRight => {
                raft_right.entity.velocity.vx = -consts::VELOCITY_GAIN_BOOST;
            }
            GameInput::DeprecatedAddProjectile => {}
            GameInput::MoveRightRaftDown => {
                raft_right.entity.velocity.vy = -consts::VELOCITY_GAIN_NORMAL;
            }
            GameInput::MoveUpRaftLeft => {
                raft_left.entity.velocity.vy = consts::VELOCITY_GAIN_NORMAL;
            }
            GameInput::MoveLeftRaftDown => {
                raft_left.entity.velocity.vy = -consts::VELOCITY_GAIN_NORMAL;
            }
        };
    }
    Ok(())
}

fn update_raft(raft: &mut Raft, curr_tick: u32) {
    let prev_entity = raft.entity.clone();
    raft.update_position(curr_tick);

    if !is_within_world_bounds(raft) {
        raft.entity = prev_entity;
    }
}

fn update_fighters(
    raft: &mut Raft,
    projectiles: &mut Vec<Projectile>,
    direction: Bearings,
    curr_tick: u32,
) {
    if raft.entity.is_active {
        for fighter in &raft.raft_fighters {
            let fire_rate = fighter.gun.fire_rate();
            if curr_tick % fire_rate == 0 {
                let proj = fighter.create_projectile(direction);
                projectiles.push(proj);
            }
        }

        raft.raft_fighters.retain(|f| f.entity.is_active);
    }
}

fn update_projectiles(projectiles: &mut Vec<Projectile>, opposing_raft: &mut Raft, curr_tick: u32) {
    for item in projectiles.iter_mut() {
        item.update_position(curr_tick);

        // Check for collisions with opposing raft fighters
        for fighter in &mut opposing_raft.raft_fighters {
            if item.collides_with(fighter) {
                fighter.take_damage(item);
                item.entity.is_active = false;
            }
        }

        // Check for collisions with the opposing raft
        if item.collides_with(opposing_raft) {
            opposing_raft.take_damage(item);
            item.entity.is_active = false;
        }

        // Check if the projectile is within world bounds
        if !is_within_world_bounds(item) {
            item.entity.is_active = false;
        }
    }

    // Retain only active projectiles
    projectiles.retain(|p| p.entity.is_active);
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

pub enum ProjectileDirection {
    ArchRight,
    ArchLeft,
    StraightDown,
}

const fn tick_inputs_needed(ticks_to_process: u32) -> u32 {
    (ticks_to_process / consts::TICKS_PER_INPUT
        + if ticks_to_process % consts::TICKS_PER_INPUT > 0 {
            1
        } else {
            0
        })
        * consts::TICK_INPUT_API_CHUNK_SIZE
}

fn is_within_world_bounds<T: Collision>(obj: &T) -> bool {
    let (x, y, width, height) = obj.bounding_box();
    x > 0
        && y > 0
        && x.saturating_add(width) <= consts::WORLD_MAX_X
        && y.saturating_add(height) <= consts::WORLD_MAX_Y
}

impl Raft {
    pub fn position_fighters(&mut self, fighters: Vec<RaftFighter>) {
        for fighter in fighters {
            let (fx, fy, fw, fh) = fighter.bounding_box();
            let (rx, ry, rw, rh) = self.bounding_box();

            // Calculate the overlapping area
            let overlap_x = (fx.max(rx) as i32 - (fx + fw).min(rx + rw) as i32).abs() as u32;
            let overlap_y = (fy.max(ry) as i32 - (fy + fh).min(ry + rh) as i32).abs() as u32;

            // Calculate the area of the fighter
            let fighter_area = fw * fh;

            // Calculate the overlapping area
            let overlap_area = overlap_x * overlap_y;

            // Check if the overlap area is at least a certain percentage of the fighter's area
            if overlap_area * 100 >= fighter_area * 5 {
                // Assuming 5% overlap is required
                self.raft_fighters.push(fighter.clone());
            }
        }
    }

    pub fn take_damage(&mut self, projectile: &Projectile) {
        if self.curr_health > 0 {
            let perc = self.max_health / self.curr_health;
            let dmg = projectile.radius * perc;

            self.curr_health = self.curr_health.saturating_sub(dmg);

            if self.curr_health == 0 {
                self.entity.is_active = false;
            }
        }
    }
}

impl RaftFighter {
    pub fn take_damage(&mut self, projectile: &Projectile) {
        if self.curr_health > 0 {
            let perc = self.max_health / self.curr_health;
            let dmg = projectile.radius * perc;

            self.curr_health = self.curr_health.saturating_sub(dmg);

            if self.curr_health == 0 {
                self.entity.is_active = false;
            }
        }
    }

    pub fn create_projectile(&self, side: Bearings) -> Projectile {
        let (radius, base_velocity) = match self.gun {
            GunTypes::Bazooka => (consts::DEFAULT_PROJECTILE_RADIUS * 2, 5),
            GunTypes::SMG => (consts::DEFAULT_PROJECTILE_RADIUS, 10),
            GunTypes::FlameThrower => (consts::DEFAULT_PROJECTILE_RADIUS, 8),
            GunTypes::StraightShooter => (consts::DEFAULT_PROJECTILE_RADIUS, 12),
        };

        let style = self.gun.style();

        let init_pos = match side {
            Bearings::East => Position {
                x: self.entity.position.x + self.width + consts::DEFAULT_PROJECTILE_RADIUS * 2,
                y: self.entity.position.y + self.height + consts::DEFAULT_PROJECTILE_RADIUS * 2,
            },
            Bearings::West => Position {
                x: self
                    .entity
                    .position
                    .x
                    .saturating_sub(consts::DEFAULT_PROJECTILE_RADIUS),
                y: self.entity.position.y + self.height + consts::DEFAULT_PROJECTILE_RADIUS * 2,
            },
            _ => Position {
                x: self.entity.position.x,
                y: self.entity.position.y,
            },
        };

        let velocity = match side {
            Bearings::North => Velocity {
                vx: 0,
                vy: -base_velocity,
            },
            Bearings::South => Velocity {
                vx: 0,
                vy: base_velocity,
            },
            Bearings::East => Velocity {
                vx: base_velocity,
                vy: 0,
            },
            Bearings::West => Velocity {
                vx: -base_velocity,
                vy: 0,
            },
            Bearings::Northeast => Velocity {
                vx: base_velocity,
                vy: -base_velocity,
            },
            Bearings::Northwest => Velocity {
                vx: -base_velocity,
                vy: -base_velocity,
            },
            Bearings::Southeast => Velocity {
                vx: base_velocity,
                vy: base_velocity,
            },
            Bearings::Southwest => Velocity {
                vx: -base_velocity,
                vy: base_velocity,
            },
        };

        Projectile {
            radius,
            entity: Entity {
                position: init_pos,
                velocity: velocity,
                is_active: true,
            },
            style,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn it_exhaustively_handles_all_game_iputs_from_() {
        for variant in GameInput::iter() {
            let as_u32 = variant.to_u32();
            let from_u32 = GameInput::from(as_u32);
            assert!(from_u32.is_ok());
            assert_eq!(from_u32.unwrap(), variant);
        }
    }

    #[test]
    fn it_creates_stable_game_state() {
        let mut state = GameState::new();
        let ticks = 1000;
        let inputs_needed = tick_inputs_needed(ticks);
        let inputs = vec![GameInput::NoOp.to_u32(); inputs_needed as usize];
        let initial_state = state.clone();

        state.tick(ticks, &inputs).unwrap();

        assert_eq!(
            initial_state.raft_left.entity.position.x,
            state.raft_left.entity.position.x
        );
        assert_eq!(
            initial_state.raft_left.entity.position.y,
            state.raft_left.entity.position.y
        );
        assert_eq!(
            initial_state.raft_right.entity.position.x,
            state.raft_right.entity.position.x
        );
        assert_eq!(
            initial_state.raft_right.entity.position.y,
            state.raft_right.entity.position.y
        );
    }

    #[test]
    #[allow(clippy::match_same_arms)]
    fn it_simulates_game_state() {
        let mut state = GameState::new();
        let ticks = 10000;
        let inputs_needed = tick_inputs_needed(ticks);
        let mut inputs = Vec::with_capacity(inputs_needed as usize);

        for i in 0..inputs_needed {
            inputs.push(match i % 33 {
                0 => GameInput::DeprecatedShootLeftRaft.to_u32(),
                1 => GameInput::MoveLeftRaftRight.to_u32(),
                2 => GameInput::MoveLeftRaftLeft.to_u32(),
                3 => GameInput::DeprecatedShootRightRaft.to_u32(),
                4 => GameInput::MoveRightRaftRight.to_u32(),
                5 => GameInput::MoveRightRaftLeft.to_u32(),
                6 => GameInput::DeprecatedAddProjectile.to_u32(),
                // repeat values so they occur more often
                7 => GameInput::DeprecatedAddProjectile.to_u32(),
                8 => GameInput::MoveLeftRaftRight.to_u32(),
                _ => GameInput::NoOp.to_u32(),
            });
        }

        state.tick(ticks, &inputs).unwrap();

        assert_eq!(state.raft_left.entity.position.x, 7497);
        assert_eq!(
            state.raft_left.entity.position.y,
            consts::LEFT_RAFT_INIT_POS.y
        );
        assert_eq!(state.raft_right.entity.position.x, 5);
        assert_eq!(
            state.raft_right.entity.position.y,
            consts::RIGHT_RAFT_INIT_POS.y
        );
    }

    #[test]
    fn it_enforces_world_bounds() {
        let mut state = GameState::new();
        let ticks = 10000;
        let inputs_needed = tick_inputs_needed(ticks);
        let inputs = vec![GameInput::MoveLeftRaftRight.to_u32(); inputs_needed as usize];
        // TODO: test projectiles respect world bounds and are correctly removed. also add in raft_right
        state.tick(ticks, &inputs).unwrap();

        assert!(state.raft_left.entity.position.x + state.raft_left.width < consts::WORLD_MAX_X);
    }
}
