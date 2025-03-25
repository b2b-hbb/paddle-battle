use serde::{Deserialize, Serialize};

use crate::consts;
use alloy_primitives::U256;
use alloy_primitives::I256;
use crate::serde_impl::{U256Def, I256Def};


#[derive(Debug, Clone, Copy)]
pub enum Bearings {
    North,
    South,
    East,
    West,
    Northeast,
    Northwest,
    Southeast,
    Southwest,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Position {
    #[serde(with = "U256Def")]
    pub x: U256,
    #[serde(with = "U256Def")]
    pub y: U256,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Velocity {
    #[serde(with = "I256Def")]
    pub vx: I256,
    #[serde(with = "I256Def")]
    pub vy: I256,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
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
    FlameThrower,
    StraightShooter,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Style {
    pub color: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Raft {
    pub entity: Entity,
    #[serde(with = "U256Def")]
    pub width: U256,
    #[serde(with = "U256Def")]
    pub height: U256,
    #[serde(with = "U256Def")]
    pub max_health: U256,
    #[serde(with = "U256Def")]
    pub curr_health: U256,
    pub raft_fighters: Vec<RaftFighter>,
    pub style: Style,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RaftFighter {
    pub entity: Entity,
    #[serde(with = "U256Def")]
    pub width: U256,
    #[serde(with = "U256Def")]
    pub height: U256,
    pub gun: GunTypes,
    #[serde(with = "U256Def")]
    pub curr_health: U256,
    #[serde(with = "U256Def")]
    pub max_health: U256,
    pub style: Style,
}

impl GunTypes {
    #[must_use]
    pub fn fire_rate(&self) -> u32 {
        match self {
            GunTypes::SMG => 20,
            GunTypes::Bazooka => 500,
            GunTypes::FlameThrower => 100,
            GunTypes::StraightShooter => 100,
        }
    }
}

impl RaftFighter {
    pub fn new(entity: Entity, gun: GunTypes, width: U256, height: U256) -> Self {
        let style = gun.style();
        Self {
            entity,
            width,
            height,
            gun,
            curr_health: U256::from(consts::DEFAULT_RAFT_HEALTH),
            max_health: U256::from(consts::DEFAULT_RAFT_HEALTH),
            style,
        }
    }
}

// #[derive(Serialize, Deserialize, Clone)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Projectile {
    pub entity: Entity,
    #[serde(with = "U256Def")]
    pub radius: U256,
    pub style: Style,
}

impl Raft {
    pub fn new(entity: Entity, style: Style) -> Self {
        Self {
            entity,
            width: U256::from(consts::DEFAULT_RAFT_WIDTH),
            height: U256::from(consts::DEFAULT_RAFT_HEIGHT),
            max_health: U256::from(consts::DEFAULT_RAFT_HEALTH),
            curr_health: U256::from(consts::DEFAULT_RAFT_HEALTH),
            raft_fighters: vec![],
            style,
        }
    }

    pub fn style(&self) -> Style {
        self.style.clone()
    }
}

impl RaftFighter {
    pub fn style(&self) -> Style {
        Style { color: "#0F002F".to_string() }
    }
}

impl GunTypes {
    pub fn style(&self) -> Style {
        let color = match self {
            GunTypes::Bazooka => "#FF0000",
            GunTypes::SMG => "#00FF00",
            GunTypes::FlameThrower => "#FFA500",
            GunTypes::StraightShooter => "#0000FF",
        };

        Style { color: color.to_string() }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct GameState {
    pub raft_left: Raft,
    pub raft_right: Raft,
    pub left_projectiles: Vec<Projectile>,
    pub right_projectiles: Vec<Projectile>,
    #[serde(with = "U256Def")]
    pub ticks: U256,
}

impl Projectile {
    pub fn new(entity: Entity, radius: U256, gun: GunTypes) -> Self {
        let style = gun.style();
        Self {
            entity,
            radius,
            style,
        }
    }
}
