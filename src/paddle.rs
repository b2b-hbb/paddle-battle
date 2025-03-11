use crate::consts;

use crate::errors::Result;
use crate::errors::SimulationError;
use crate::physics::Collision;
use crate::world::GunTypes;
use crate::world::RaftFighter;
use crate::world::{Entity, GameState, Position, Projectile, Raft, Velocity};

#[cfg(test)]
use strum_macros::{EnumCount, EnumIter};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq, EnumIter, EnumCount))]
pub enum GameInput {
    NoOp,
    ShootLeftRaft,
    MoveLeftRaftRight,
    MoveLeftRaftLeft,
    ShootRightRaft,
    MoveRightRaftRight,
    MoveRightRaftLeft,
    AddProjectile,
    MoveLeftFastRaftRight,
    MoveLeftFastRaftLeft,
}

impl GameInput {
    /// # Errors
    ///
    /// Will return `Err` if `input` does not exist as a valid game input
    pub const fn from(input: u32) -> Result<Self> {
        match input {
            0 => Ok(Self::ShootLeftRaft),
            1 => Ok(Self::MoveLeftRaftRight),
            2 => Ok(Self::MoveLeftRaftLeft),
            3 => Ok(Self::ShootRightRaft),
            4 => Ok(Self::MoveRightRaftRight),
            5 => Ok(Self::MoveRightRaftLeft),
            6 => Ok(Self::AddProjectile),
            7 => Ok(Self::MoveLeftFastRaftLeft),
            8 => Ok(Self::MoveLeftFastRaftRight),
            86 => Ok(Self::NoOp),
            received => Err(SimulationError::InvalidInput { received }),
        }
    }

    #[must_use]
    pub const fn to_u32(&self) -> u32 {
        match self {
            Self::ShootLeftRaft => 0,
            Self::MoveLeftRaftRight => 1,
            Self::MoveLeftRaftLeft => 2,
            Self::ShootRightRaft => 3,
            Self::MoveRightRaftRight => 4,
            Self::MoveRightRaftLeft => 5,
            Self::AddProjectile => 6,
            Self::MoveLeftFastRaftLeft => 7,
            Self::MoveLeftFastRaftRight => 8,
            Self::NoOp => 86,
        }
    }
}

impl GameState {
    #[must_use]
    pub fn new() -> Self {
        let left_fighters = vec![RaftFighter::new(
            Entity {
                position: consts::LEFT_RAFT_INIT_POS,
                velocity: consts::NO_VELOCITY,
                is_active: true,
            },
            GunTypes::Bazooka,
        )];
        let mut right_fighters = vec![RaftFighter::new(
            Entity {
                position: consts::RIGHT_RAFT_INIT_POS,
                velocity: consts::NO_VELOCITY,
                is_active: true,
            },
            GunTypes::SMG,
        )];
        right_fighters[0].entity.position.x = right_fighters[0]
            .entity
            .position
            .x
            .saturating_add(consts::DEFAULT_RAFT_WIDTH);
        Self {
            raft_left: Raft::new(
                Entity {
                    position: consts::LEFT_RAFT_INIT_POS,
                    velocity: consts::NO_VELOCITY,
                    is_active: true,
                },
                GunTypes::Bazooka,
                left_fighters,
            ),
            raft_right: Raft::new(
                Entity {
                    position: consts::RIGHT_RAFT_INIT_POS,
                    velocity: consts::NO_VELOCITY,
                    is_active: true,
                },
                GunTypes::SMG,
                right_fighters,
            ),
            projectiles: vec![],
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
            let projectiles = &mut self.projectiles;

            // handle input
            if curr_tick % consts::TICKS_PER_INPUT == 0 {
                match iter.next() {
                    None => {
                        return Err(SimulationError::NoInput {
                            num_tick: curr_tick,
                        })
                    }
                    Some(input_for_tick) => {
                        for &curr in input_for_tick {
                            match GameInput::from(curr) {
                                Err(e) => return Err(e),
                                Ok(input) => match input {
                                    GameInput::NoOp => {}
                                    GameInput::ShootLeftRaft => {
                                        let prj = Projectile::create_projectile(
                                            raft_left.shoots_from(&ShootsFromSide::Right),
                                            &ProjectileDirection::ArchRight,
                                            &GunTypes::Bazooka,
                                        );

                                        projectiles.push(prj);
                                    }
                                    GameInput::ShootRightRaft => {
                                        let prj = Projectile::create_projectile(
                                            raft_right.shoots_from(&ShootsFromSide::Left),
                                            &ProjectileDirection::ArchLeft,
                                            &GunTypes::SMG,
                                        );
                                        projectiles.push(prj);
                                    }
                                    GameInput::MoveLeftRaftRight => {
                                        raft_left.entity.velocity.vx = consts::VELOCITY_GAIN_NORMAL;
                                    }
                                    GameInput::MoveLeftRaftLeft => {
                                        raft_left.entity.velocity.vx =
                                            -consts::VELOCITY_GAIN_NORMAL;
                                    }
                                    GameInput::MoveRightRaftRight => {
                                        raft_right.entity.velocity.vx =
                                            consts::VELOCITY_GAIN_NORMAL;
                                    }
                                    GameInput::MoveRightRaftLeft => {
                                        raft_right.entity.velocity.vx =
                                            -consts::VELOCITY_GAIN_NORMAL;
                                    }
                                    GameInput::MoveLeftFastRaftLeft => {
                                        raft_left.entity.velocity.vx = consts::VELOCITY_GAIN_BOOST;
                                    }
                                    GameInput::MoveLeftFastRaftRight => {
                                        raft_right.entity.velocity.vx =
                                            -consts::VELOCITY_GAIN_BOOST;
                                    }
                                    GameInput::AddProjectile => {
                                        let prj = Projectile::create_projectile(
                                            Position {
                                                x: consts::WORLD_MAX_X / 2,
                                                y: consts::WORLD_MAX_Y
                                                    - consts::DEFAULT_PROJECTILE_RADIUS * 2
                                                    - 1,
                                            },
                                            &ProjectileDirection::StraightDown,
                                            &GunTypes::SMG,
                                        );
                                        projectiles.push(prj);
                                    }
                                },
                            };
                        }
                    }
                }
            }

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

            if raft_left.entity.is_active {
                // TODO: assess alternative to avoid clone of fighters
                let prev_entity = raft_left.entity.clone();
                raft_left.update_position(curr_tick);

                if !is_within_world_bounds(raft_left) {
                    raft_left.entity = prev_entity;
                }

                // TODO: make sure fighters move alongside the raft
                for fighter in &raft_left.raft_fighters {
                    let fire_rate = match fighter.gun {
                        GunTypes::SMG => 20,
                        GunTypes::Bazooka => 500,
                    };

                    if curr_tick % fire_rate == 0 {
                        let init_pos = fighter.shoots_from(&ShootsFromSide::Right);
                        let proj = Projectile::create_projectile(
                            init_pos,
                            &ProjectileDirection::ArchRight,
                            &fighter.gun,
                        );
                        projectiles.push(proj);
                    }
                }
            }

            if raft_right.entity.is_active {
                let prev_entity = raft_right.entity.clone();
                raft_right.update_position(curr_tick);

                if !is_within_world_bounds(raft_right) {
                    raft_right.entity = prev_entity;
                }

                // TODO: make sure fighters move alongside the raft
                for fighter in &raft_right.raft_fighters {
                    let fire_rate = match fighter.gun {
                        GunTypes::SMG => 20,
                        GunTypes::Bazooka => 500,
                    };

                    if curr_tick % fire_rate == 0 {
                        let init_pos = fighter.shoots_from(&ShootsFromSide::Left);
                        let proj = Projectile::create_projectile(
                            init_pos,
                            &ProjectileDirection::ArchLeft,
                            &fighter.gun,
                        );
                        projectiles.push(proj);
                    }
                }
            }

            for item in &mut *projectiles {
                item.update_position(curr_tick);

                // TODO: do quick x check to separate left and right and cut collision checks in half
                for fighter in &mut *raft_left.raft_fighters {
                    if item.collides_with(fighter) {
                        fighter.curr_health = fighter
                            .curr_health
                            .saturating_sub(item.calculate_dmg_fighter(fighter));
                        item.entity.is_active = false;
                    }
                }

                for fighter in &mut *raft_right.raft_fighters {
                    if item.collides_with(fighter) {
                        fighter.curr_health = fighter
                            .curr_health
                            .saturating_sub(item.calculate_dmg_fighter(fighter));
                        item.entity.is_active = false;
                    }
                }

                if item.collides_with(raft_right) {
                    raft_right.curr_health = raft_right
                        .curr_health
                        .saturating_sub(item.calculate_dmg(raft_right));
                    item.entity.is_active = false;
                }

                if item.collides_with(raft_left) {
                    raft_left.curr_health = raft_left
                        .curr_health
                        .saturating_sub(item.calculate_dmg(raft_left));
                    item.entity.is_active = false;
                }

                if !is_within_world_bounds(item) {
                    item.entity.is_active = false;
                }
            }

            projectiles.retain(|p| p.entity.is_active);

            self.ticks += 1;
        }
        Ok(())
    }
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
impl Projectile {
    #[must_use]
    pub const fn create_projectile(
        init_pos: Position,
        direction: &ProjectileDirection,
        gun: &GunTypes,
    ) -> Self {
        // TODO: vary speed depending on gun type
        let vx = consts::WORLD_MAX_X_I32 * 5 / 1000;
        let vy = consts::WORLD_MAX_Y_I32 * 5 / 1000;
        let radius = match gun {
            GunTypes::Bazooka => consts::DEFAULT_PROJECTILE_RADIUS * 2,
            GunTypes::SMG => consts::DEFAULT_PROJECTILE_RADIUS,
        };

        Self {
            radius,
            entity: Entity {
                position: init_pos,
                velocity: match direction {
                    ProjectileDirection::ArchLeft => Velocity { vx: -vx, vy },
                    ProjectileDirection::ArchRight => Velocity { vx, vy },
                    ProjectileDirection::StraightDown => Velocity { vx: 0, vy: -vy },
                },
                is_active: true,
            },
        }
    }

    #[must_use]
    pub const fn calculate_dmg(&self, raft: &Raft) -> u32 {
        if raft.curr_health > 0 {
            let perc = raft.max_health / raft.curr_health;
            self.radius * perc
        } else {
            0
        }
    }

    // TODO: DRY up this code with raft calculate dmg above
    #[must_use]
    pub const fn calculate_dmg_fighter(&self, raft_fighter: &RaftFighter) -> u32 {
        if raft_fighter.curr_health > 0 {
            let perc = raft_fighter.max_health / raft_fighter.curr_health;
            self.radius * perc
        } else {
            0
        }
    }
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
    x > 0 && y > 0 && x + width < consts::WORLD_MAX_X && y + height < consts::WORLD_MAX_Y
}

pub enum ShootsFromSide {
    Left,
    Right,
}

impl Raft {
    #[must_use]
    pub const fn shoots_from(&self, side: &ShootsFromSide) -> Position {
        match side {
            ShootsFromSide::Right => Position {
                // TODO: adjust radius depending on gun type
                x: self.entity.position.x + self.width + consts::DEFAULT_PROJECTILE_RADIUS * 2,
                y: self.entity.position.y + self.height + consts::DEFAULT_PROJECTILE_RADIUS * 2,
            },
            ShootsFromSide::Left => Position {
                x: self
                    .entity
                    .position
                    .x
                    .saturating_sub(consts::DEFAULT_PROJECTILE_RADIUS),
                y: self.entity.position.y + self.height + consts::DEFAULT_PROJECTILE_RADIUS * 2,
            },
        }
    }
}

impl RaftFighter {
    // TODO: DRY UP
    #[must_use]
    pub const fn shoots_from(&self, side: &ShootsFromSide) -> Position {
        match side {
            ShootsFromSide::Right => Position {
                x: self.entity.position.x + self.width + consts::DEFAULT_PROJECTILE_RADIUS * 2,
                y: self.entity.position.y + self.height + consts::DEFAULT_PROJECTILE_RADIUS * 2,
            },
            ShootsFromSide::Left => Position {
                x: self
                    .entity
                    .position
                    .x
                    .saturating_sub(consts::DEFAULT_PROJECTILE_RADIUS),
                y: self.entity.position.y + self.height + consts::DEFAULT_PROJECTILE_RADIUS * 2,
            },
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
                0 => GameInput::ShootLeftRaft.to_u32(),
                1 => GameInput::MoveLeftRaftRight.to_u32(),
                2 => GameInput::MoveLeftRaftLeft.to_u32(),
                3 => GameInput::ShootRightRaft.to_u32(),
                4 => GameInput::MoveRightRaftRight.to_u32(),
                5 => GameInput::MoveRightRaftLeft.to_u32(),
                6 => GameInput::AddProjectile.to_u32(),
                // repeat values so they occur more often
                7 => GameInput::AddProjectile.to_u32(),
                8 => GameInput::MoveLeftRaftRight.to_u32(),
                _ => GameInput::NoOp.to_u32(),
            });
        }

        state.tick(ticks, &inputs).unwrap();

        assert_eq!(state.raft_left.entity.position.x, 8996);
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
