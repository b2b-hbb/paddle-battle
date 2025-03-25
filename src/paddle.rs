use crate::consts;

use crate::errors::Result;
use crate::errors::SimulationError;
use crate::physics::Collision;
use crate::world::Bearings;
use crate::world::GunTypes;
use crate::world::RaftFighter;
use crate::world::{Entity, GameState, Position, Projectile, Raft, Velocity};
use crate::world::Style;

use alloy_primitives::I256;
use alloy_primitives::U256;
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
    pub fn from(input: U256) -> Result<Self> {
        match input {
            x if x == U256::from(0) => Ok(Self::DeprecatedShootLeftRaft),
            x if x == U256::from(1) => Ok(Self::MoveLeftRaftRight),
            x if x == U256::from(2) => Ok(Self::MoveLeftRaftLeft),
            x if x == U256::from(3) => Ok(Self::DeprecatedShootRightRaft),
            x if x == U256::from(4) => Ok(Self::MoveRightRaftRight),
            x if x == U256::from(5) => Ok(Self::MoveRightRaftLeft),
            x if x == U256::from(6) => Ok(Self::DeprecatedAddProjectile),
            x if x == U256::from(7) => Ok(Self::MoveLeftFastRaftLeft),
            x if x == U256::from(8) => Ok(Self::MoveLeftFastRaftRight),
            x if x == U256::from(9) => Ok(Self::MoveUpRaftRight),
            x if x == U256::from(10) => Ok(Self::MoveRightRaftDown),
            x if x == U256::from(11) => Ok(Self::MoveUpRaftLeft),
            x if x == U256::from(12) => Ok(Self::MoveLeftRaftDown),
            x if x == U256::from(86) => Ok(Self::NoOp),
            received => Err(SimulationError::InvalidInput { received }),
        }
    }

    #[must_use]
    pub fn to_U256(&self) -> U256 {
        match self {
            Self::DeprecatedShootLeftRaft => U256::from(0),
            Self::MoveLeftRaftRight => U256::from(1),
            Self::MoveLeftRaftLeft => U256::from(2),
            Self::DeprecatedShootRightRaft => U256::from(3),
            Self::MoveRightRaftRight => U256::from(4),
            Self::MoveRightRaftLeft => U256::from(5),
            Self::DeprecatedAddProjectile => U256::from(6),
            Self::MoveLeftFastRaftLeft => U256::from(7),
            Self::MoveLeftFastRaftRight => U256::from(8),
            Self::MoveUpRaftRight => U256::from(9),
            Self::MoveRightRaftDown => U256::from(10),
            Self::MoveUpRaftLeft => U256::from(11),
            Self::MoveLeftRaftDown => U256::from(12),
            Self::NoOp => U256::from(86),
        }
    }
}

impl GameState {
    #[must_use]
    pub fn new() -> Self {

        let mut raft_left = Raft::new(
            Entity {
                position: consts::left_raft_init_pos(),
                velocity: consts::no_velocity(),
                is_active: true,
            },
            Style { color: "#FF0000".to_string() },
        );

        let fighter_entity = Entity {
            position: Position {
                x: raft_left.entity.position.x + raft_left.width * U256::from(4) / U256::from(5),
                y: raft_left.entity.position.y + raft_left.height * U256::from(4) / U256::from(5),
            },
            velocity: consts::no_velocity(),
            is_active: true,
        };

        let bazooka_fighter = RaftFighter::new(
            fighter_entity, 
            GunTypes::SMG,
            U256::from(consts::DEFAULT_RAFT_FIGHTER_WIDTH),
            U256::from(consts::DEFAULT_RAFT_FIGHTER_HEIGHT)
        );

        raft_left.position_fighters(vec![bazooka_fighter]);

        Self {
            raft_left,
            raft_right: Raft::new(
                Entity {
                    position: Position {
                        x: U256::from(consts::WORLD_MAX_X) - U256::from(consts::DEFAULT_RAFT_WIDTH),
                        y: U256::from(consts::right_raft_init_pos().y),
                    },
                    velocity: consts::no_velocity(),
                    is_active: true,
                },
                Style { color: "#0000FF".to_string() },
            ),
            left_projectiles: vec![],
            right_projectiles: vec![],
            ticks: U256::from(0),
        }
    }

    /// # Errors
    ///
    /// Will return Err if input is invalid
    #[allow(clippy::too_many_lines)]
    pub fn tick(&mut self, ticks_to_process: U256, input: &Vec<U256>) -> Result<()> {
        let initial_tick = self.ticks;
        let end_tick = initial_tick + ticks_to_process;
        let inputs_needed = tick_inputs_needed(ticks_to_process);
        let input_len = U256::from(input.len());

        if input_len != inputs_needed {
            return Err(SimulationError::InvalidInputLength {
                received: U256::from(input_len),
                expected: U256::from(inputs_needed),
                initial_tick: U256::from(initial_tick),
                end_tick: U256::from(end_tick),
            });
        }

        // TODO: does chunk and iterator result in same performance than manually calculating index?
        // let input_index = (tick / consts::TICKS_PER_INPUT) as usize - current_input_index;
        let chunked_input = input
            .chunks(consts::TICK_INPUT_API_CHUNK_SIZE as usize)
            .map(<[U256]>::to_vec)
            .collect::<Vec<Vec<U256>>>();
        let mut iter = chunked_input.iter();

        for curr_tick in initial_tick.to::<u32>() .. end_tick.to::<u32>() {
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
                handle_input(iter.next(), raft_left, raft_right, U256::from(curr_tick))?;
            }

            update_raft(raft_left, U256::from(curr_tick));
            update_raft(raft_right, U256::from(curr_tick));

            update_fighters(raft_left, &mut self.left_projectiles, Bearings::Northeast, curr_tick);
            update_fighters(raft_right, &mut self.right_projectiles, Bearings::Northwest, curr_tick);

            update_projectiles(&mut self.left_projectiles, raft_right, U256::from(curr_tick));
            update_projectiles(&mut self.right_projectiles, raft_left, U256::from(curr_tick));

            self.ticks += U256::from(1);
        }
        Ok(())
    }
}


fn handle_input(
    input_for_tick: Option<&Vec<U256>>,
    raft_left: &mut Raft,
    raft_right: &mut Raft,
    curr_tick: U256,
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
                raft_left.entity.velocity.vx = I256::from_dec_str(consts::VELOCITY_GAIN_NORMAL).unwrap();
            }
            GameInput::MoveLeftRaftLeft => {
                raft_left.entity.velocity.vx =
                    I256::from_dec_str(consts::VELOCITY_GAIN_NORMAL).unwrap() *
                    I256::from_dec_str("-1").unwrap();
            }

            GameInput::MoveRightRaftRight => {
                raft_right.entity.velocity.vx = I256::from_dec_str(consts::VELOCITY_GAIN_NORMAL).unwrap();
            }
            GameInput::MoveUpRaftRight => {
                raft_right.entity.velocity.vy = I256::from_dec_str(consts::VELOCITY_GAIN_NORMAL).unwrap();
            }
            GameInput::MoveRightRaftLeft => {
                raft_right.entity.velocity.vx =
                I256::from_dec_str(consts::VELOCITY_GAIN_NORMAL).unwrap() *
                I256::from_dec_str("-1").unwrap();
            }
            GameInput::MoveLeftFastRaftLeft => {
                raft_left.entity.velocity.vx = I256::from_dec_str(consts::VELOCITY_GAIN_BOOST).unwrap();
            }
            GameInput::MoveLeftFastRaftRight => {
                raft_right.entity.velocity.vx = 
                I256::from_dec_str(consts::VELOCITY_GAIN_BOOST).unwrap() *
                I256::from_dec_str("-1").unwrap();
            }
            GameInput::DeprecatedAddProjectile => {}
            GameInput::MoveRightRaftDown => {
                raft_right.entity.velocity.vy =
                I256::from_dec_str(consts::VELOCITY_GAIN_NORMAL).unwrap() *
                I256::from_dec_str("-1").unwrap();
            }
            GameInput::MoveUpRaftLeft => {
                raft_left.entity.velocity.vy =
                I256::from_dec_str(consts::VELOCITY_GAIN_NORMAL).unwrap();
            }
            GameInput::MoveLeftRaftDown => {
                raft_left.entity.velocity.vy = I256::from_dec_str(consts::VELOCITY_GAIN_NORMAL).unwrap() *
                I256::from_dec_str("-1").unwrap();;
            }
        };
    }
    Ok(())
}

fn update_raft(raft: &mut Raft, curr_tick: U256) {
    let prev_entity = raft.entity.clone();
    raft.update_position(curr_tick);

    if !is_within_world_bounds(raft) {
        raft.entity = prev_entity;
    }
}

fn update_fighters(raft: &mut Raft, projectiles: &mut Vec<Projectile>, direction: Bearings,curr_tick: u32) {
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

fn update_projectiles(projectiles: &mut Vec<Projectile>, opposing_raft: &mut Raft, curr_tick: U256) {
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

fn tick_inputs_needed(ticks_to_process: U256) -> U256 {
    (ticks_to_process / U256::from(consts::TICKS_PER_INPUT)
        + if ticks_to_process % U256::from(consts::TICKS_PER_INPUT) > U256::ZERO {
            U256::from(1)
        } else {
            U256::ZERO
        })
        * U256::from(consts::TICK_INPUT_API_CHUNK_SIZE)
}

fn is_within_world_bounds<T: Collision>(obj: &T) -> bool {
    let (x, y, width, height) = obj.bounding_box();

    x > U256::ZERO &&
    y > U256::ZERO &&
    x.saturating_add(width) <= U256::from(consts::WORLD_MAX_X) &&
    y.saturating_add(height) <= U256::from(consts::WORLD_MAX_Y)
}


impl Raft {
    pub fn position_fighters(&mut self, fighters: Vec<RaftFighter>) {
        for fighter in fighters {
            let (fx, fy, fw, fh) = fighter.bounding_box();
            let (rx, ry, rw, rh) = self.bounding_box();

            self.raft_fighters.push(fighter.clone());

            /* 
            // Calculate the overlapping area
            let overlap_x = (fx.max(rx) as i32 - (fx + fw).min(rx + rw) as i32).abs() as u32;
            let overlap_y = (fy.max(ry) as i32 - (fy + fh).min(ry + rh) as i32).abs() as u32;

            // Calculate the area of the fighter
            let fighter_area = fw * fh;

            // Calculate the overlapping area
            let overlap_area = overlap_x * overlap_y;

            // Check if the overlap area is at least a certain percentage of the fighter's area
            if overlap_area * 100 >= fighter_area * 5 { // Assuming 5% overlap is required
                self.raft_fighters.push(fighter.clone());
            }
            */
        }
    }

    pub fn take_damage(&mut self, projectile: &Projectile) {
        if self.curr_health > U256::ZERO {
            let perc = self.max_health / self.curr_health;
            let dmg = projectile.radius * perc;
            
            self.curr_health = self.curr_health.saturating_sub(dmg);
            
            if self.curr_health == U256::ZERO {
                self.entity.is_active = false;
            }
        }
    }
}

impl RaftFighter {
    pub fn take_damage(&mut self, projectile: &Projectile) {
        if self.curr_health > U256::ZERO {
            let perc = self.max_health / self.curr_health;
            let dmg = projectile.radius * perc;
            
            self.curr_health = self.curr_health.saturating_sub(dmg);
            
            if self.curr_health == U256::ZERO {
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
                x: self.entity.position.x + self.width + U256::from(consts::DEFAULT_PROJECTILE_RADIUS * 2),
                y: self.entity.position.y + self.height + U256::from(consts::DEFAULT_PROJECTILE_RADIUS * 2),
            },
            Bearings::West => Position {
                x: self
                    .entity
                    .position
                    .x
                    .saturating_sub(U256::from(consts::DEFAULT_PROJECTILE_RADIUS)),
                y: self.entity.position.y + self.height + U256::from(consts::DEFAULT_PROJECTILE_RADIUS * 2),
            },
            _ => Position {
                x: self.entity.position.x,
                y: self.entity.position.y,
            },
        };

        let positive_base_velocity = I256::from_dec_str(&base_velocity.to_string()).unwrap();
        let negative_base_velocity = I256::from_dec_str(&(-base_velocity).to_string()).unwrap();
        
        let velocity = match side {
            Bearings::North => Velocity { vx: I256::ZERO, vy: negative_base_velocity },
            Bearings::South => Velocity { vx: I256::ZERO, vy: positive_base_velocity },
            Bearings::East => Velocity { vx: positive_base_velocity, vy: I256::ZERO },
            Bearings::West => Velocity { vx: negative_base_velocity, vy: I256::ZERO },
            Bearings::Northeast => Velocity { vx: positive_base_velocity, vy: negative_base_velocity },
            Bearings::Northwest => Velocity { vx: negative_base_velocity, vy: negative_base_velocity },
            Bearings::Southeast => Velocity { vx: positive_base_velocity, vy: positive_base_velocity },
            Bearings::Southwest => Velocity { vx: negative_base_velocity, vy: positive_base_velocity },
        };

        Projectile {
            radius: U256::from(radius),
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
            let as_u32 = variant.to_U256();
            let from_u32 = GameInput::from(as_u32);
            assert!(from_u32.is_ok());
            assert_eq!(from_u32.unwrap(), variant);
        }
    }

    #[test]
    fn it_creates_stable_game_state() {
        let mut state = GameState::new();
        let ticks = U256::from(1000);
        let inputs_needed = tick_inputs_needed(ticks);
        let inputs = vec![GameInput::NoOp.to_U256(); inputs_needed.to::<u32>() as usize];
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
        let ticks = U256::from(10000);
        let inputs_needed = tick_inputs_needed(ticks);
        let mut inputs = Vec::with_capacity(inputs_needed.to::<u32>() as usize);

        for i in 0..inputs_needed.to::<u32>() {
            inputs.push(match i % 33 {
                0 => GameInput::DeprecatedShootLeftRaft.to_U256(),
                1 => GameInput::MoveLeftRaftRight.to_U256(),
                2 => GameInput::MoveLeftRaftLeft.to_U256(),
                3 => GameInput::DeprecatedShootRightRaft.to_U256(),
                4 => GameInput::MoveRightRaftRight.to_U256(),
                5 => GameInput::MoveRightRaftLeft.to_U256(),
                6 => GameInput::DeprecatedAddProjectile.to_U256(),
                // repeat values so they occur more often
                7 => GameInput::DeprecatedAddProjectile.to_U256(),
                8 => GameInput::MoveLeftRaftRight.to_U256(),
                _ => GameInput::NoOp.to_U256(),
            });
        }

        state.tick(ticks, &inputs).unwrap();

        assert_eq!(state.raft_left.entity.position.x, U256::from(7497));
        assert_eq!(
            state.raft_left.entity.position.y,
            consts::left_raft_init_pos().y
        );
        assert_eq!(state.raft_right.entity.position.x, U256::from(5));
        assert_eq!(
            state.raft_right.entity.position.y,
            consts::right_raft_init_pos().y
        );
    }

    #[test]
    fn it_enforces_world_bounds() {
        let mut state = GameState::new();
        let ticks = U256::from(10000);
        let inputs_needed = tick_inputs_needed(ticks);
        let inputs = vec![GameInput::MoveLeftRaftRight.to_U256(); inputs_needed.to::<u32>() as usize];
        // TODO: test projectiles respect world bounds and are correctly removed. also add in raft_right
        state.tick(ticks, &inputs).unwrap();

        assert!(state.raft_left.entity.position.x + state.raft_left.width < U256::from(consts::WORLD_MAX_X));
    }
}
