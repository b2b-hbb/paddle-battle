extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use alloc::vec;
use alloy_primitives::B256;
use alloy_primitives::Bytes;
use alloy_primitives::keccak256;
use minicbor::{Encode, Decode};

use crate::consts;

#[derive(Debug, Clone, Copy, Encode, Decode)]
#[cbor(map)]
pub enum Bearings {
    #[n(0)]
    North,
    #[n(1)]
    South,
    #[n(2)]
    East,
    #[n(3)]
    West,
    #[n(4)]
    Northeast,
    #[n(5)]
    Northwest,
    #[n(6)]
    Southeast,
    #[n(7)]
    Southwest,
}

#[derive(Clone, Encode, Decode)]
#[cbor(map)]
pub struct Position {
    #[n(0)]
    pub x: u32,
    #[n(1)]
    pub y: u32,
}

#[derive(Clone, Encode, Decode)]
#[cbor(map)]
pub struct Velocity {
    #[n(0)]
    pub vx: i32,
    #[n(1)]
    pub vy: i32,
}

#[derive(Clone, Encode, Decode)]
#[cbor(map)]
pub struct Entity {
    #[n(0)]
    pub position: Position,
    #[n(1)]
    pub velocity: Velocity,
    #[n(2)]
    pub is_active: bool,
}

#[derive(Clone, Encode, Decode)]
#[cbor(map)]
pub enum GunTypes {
    #[n(0)]
    Bazooka,
    #[n(1)]
    SMG,
    #[n(2)]
    FlameThrower,
    #[n(3)]
    StraightShooter,
}

#[derive(Clone, Encode, Decode)]
#[cbor(map)]
pub struct Style {
    #[n(0)]
    pub color: String,
}

#[derive(Clone, Encode, Decode)]
#[cbor(map)]
pub struct Raft {
    #[n(0)]
    pub entity: Entity,
    #[n(1)]
    pub width: u32,
    #[n(2)]
    pub height: u32,
    #[n(3)]
    pub max_health: u32,
    #[n(4)]
    pub curr_health: u32,
    #[n(5)]
    pub raft_fighters: Vec<RaftFighter>,
    #[n(6)]
    pub style: Style,
}

#[derive(Clone, Encode, Decode)]
#[cbor(map)]
pub struct RaftFighter {
    #[n(0)]
    pub entity: Entity,
    #[n(1)]
    pub width: u32,
    #[n(2)]
    pub height: u32,
    #[n(3)]
    pub gun: GunTypes,
    #[n(4)]
    pub curr_health: u32,
    #[n(5)]
    pub max_health: u32,
    #[n(6)]
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

#[derive(Clone, Encode, Decode)]
#[cbor(map)]
pub struct Projectile {
    #[n(0)]
    pub entity: Entity,
    #[n(1)]
    pub radius: u32,
    #[n(2)]
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
            color: String::from("#0F002F"),
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
            color: String::from(color),
        }
    }
}

#[derive(Clone, Encode, Decode)]
#[cbor(map)]
pub struct GameState {
    #[n(0)]
    pub raft_left: Raft,
    #[n(1)]
    pub raft_right: Raft,
    #[n(2)]
    pub left_projectiles: Vec<Projectile>,
    #[n(3)]
    pub right_projectiles: Vec<Projectile>,
    #[n(4)]
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

impl GameState {
    pub fn to_serialized_state(&self) -> Vec<u8> {
        minicbor::to_vec(self).expect("CBOR encoding failed")
    }

    pub fn from_serialized_state(encoded: &Vec<u8>) -> Self {
        minicbor::decode(encoded).expect("CBOR decoding failed")
    }
    // pub fn to_serialized_state(&self) -> Bytes {
    //     Bytes::from(minicbor::to_vec(self).expect("CBOR encoding failed"))
    // }

    // pub fn from_serialized_state(encoded: &Bytes) -> Self {
    //     minicbor::decode(encoded).expect("CBOR decoding failed")
    // }

    pub fn hash(&self) -> B256 {
        let serialized_game_state = self.to_serialized_state();
        let hash = keccak256(&serialized_game_state);
        hash
    }
}
