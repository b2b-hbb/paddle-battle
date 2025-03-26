use serde::{Deserialize, Serialize};

use crate::consts;

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
    pub width: u32,
    pub height: u32,
    pub max_health: u32,
    pub curr_health: u32,
    pub raft_fighters: Vec<RaftFighter>,
    pub style: Style,
}

// Update RaftFighter to include health tracking
#[derive(Serialize, Deserialize, Clone)]
pub struct RaftFighter {
    pub entity: Entity,
    pub width: u32,
    pub height: u32,
    pub gun: GunTypes,
    pub curr_health: u32,
    pub max_health: u32,
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
    pub fn new(entity: Entity, gun: GunTypes, width: u32, height: u32) -> Self {
        let style = gun.style();
        Self {
            entity,
            width,
            height,
            gun,
            curr_health: consts::DEFAULT_RAFT_HEALTH,
            max_health: consts::DEFAULT_RAFT_HEALTH,
            style,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Projectile {
    pub entity: Entity,
    pub radius: u32,
    pub style: Style,
}

impl Raft {
    pub fn new(entity: Entity, style: Style) -> Self {
        Self {
            entity,
            width: consts::DEFAULT_RAFT_WIDTH,
            height: consts::DEFAULT_RAFT_HEIGHT,
            max_health: consts::DEFAULT_RAFT_HEALTH,
            curr_health: consts::DEFAULT_RAFT_HEALTH,
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
        Style {
            color: "#0F002F".to_string(),
        }
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

        Style {
            color: color.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub raft_left: Raft,
    pub raft_right: Raft,
    pub left_projectiles: Vec<Projectile>,
    pub right_projectiles: Vec<Projectile>,
    pub ticks: u32,
}

impl Projectile {
    pub fn new(entity: Entity, radius: u32, gun: GunTypes) -> Self {
        let style = gun.style();
        Self {
            entity,
            radius,
            style,
        }
    }
}
