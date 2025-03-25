#[derive(Debug, PartialEq, Eq)]
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

pub const TICKS_PER_INPUT: u32 = 5;
pub const TICK_INPUT_API_CHUNK_SIZE: u32 = 10; 