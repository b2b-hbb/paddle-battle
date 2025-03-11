use serde::{Deserialize, Serialize};

use crate::consts;

#[derive(Serialize, Deserialize, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Velocity {
    pub vx: i32,
    pub vy: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Entity {
    pub position: Position,
    pub velocity: Velocity,
    pub is_active: bool,
}

// TODO: add different firing rates for different guns
#[derive(Serialize, Deserialize, Clone)]
pub enum GunTypes {
    Bazooka,
    SMG,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Raft {
    pub entity: Entity,
    pub width: u32,
    pub height: u32,
    pub max_health: u32,
    pub curr_health: u32,
    pub gun: GunTypes,
    pub raft_fighters: Vec<RaftFighter>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RaftFighter {
    pub entity: Entity,
    pub width: u32,
    pub height: u32,
    pub max_health: u32,
    pub curr_health: u32,
    pub gun: GunTypes,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Projectile {
    pub entity: Entity,
    pub radius: u32,
}

impl Raft {
    #[must_use]
    pub const fn new(entity: Entity, gun: GunTypes, raft_fighters: Vec<RaftFighter>) -> Self {
        Self {
            entity,
            width: consts::DEFAULT_RAFT_WIDTH,
            height: consts::DEFAULT_RAFT_HEIGHT,
            max_health: consts::DEFAULT_RAFT_HEALTH,
            curr_health: consts::DEFAULT_RAFT_HEALTH,
            raft_fighters,
            gun,
        }
    }
}

impl RaftFighter {
    #[must_use]
    pub const fn new(entity: Entity, gun: GunTypes) -> Self {
        match gun {
            GunTypes::Bazooka => Self {
                entity,
                width: consts::DEFAULT_RAFT_WIDTH / 3,
                height: consts::DEFAULT_RAFT_HEIGHT * 5,
                max_health: consts::DEFAULT_RAFT_HEALTH * 3,
                curr_health: consts::DEFAULT_RAFT_HEALTH * 3,
                gun,
            },
            GunTypes::SMG => Self {
                entity,
                width: consts::DEFAULT_RAFT_WIDTH / 7,
                height: consts::DEFAULT_RAFT_HEIGHT * 2,
                max_health: consts::DEFAULT_RAFT_HEALTH,
                curr_health: consts::DEFAULT_RAFT_HEALTH,
                gun,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub raft_left: Raft,
    pub raft_right: Raft,
    pub projectiles: Vec<Projectile>,
    pub ticks: u32,
}
