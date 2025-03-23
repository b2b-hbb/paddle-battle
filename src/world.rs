use serde::{Deserialize, Serialize};

use crate::consts;
use crate::consts::DEFAULT_RAFT_HEIGHT;
use crate::physics::Collision;

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
    pub gun: GunTypes,
    pub raft_fighters: Vec<RaftFighter>,
    pub style: Style,
}

// Define projectile types
pub enum ProjectileType {
    Small,
    Medium,
    Large,
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

    pub fn take_damage(&mut self, damage: u32) {
        self.curr_health = self.curr_health.saturating_sub(damage);
        if self.curr_health == 0 {
            self.entity.is_active = false;
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
    pub fn new(entity: Entity, gun: GunTypes, style: Style) -> Self {
        Self {
            entity,
            width: consts::DEFAULT_RAFT_WIDTH,
            height: consts::DEFAULT_RAFT_HEIGHT,
            max_health: consts::DEFAULT_RAFT_HEALTH,
            curr_health: consts::DEFAULT_RAFT_HEALTH,
            gun,
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
        };

        Style { color: color.to_string() }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub raft_left: Raft,
    pub raft_right: Raft,
    pub projectiles: Vec<Projectile>,
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
