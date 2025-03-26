use core::fmt;

pub type Result<T> = core::result::Result<T, SimulationError>;

#[derive(Debug)]
pub enum SimulationError {
    InvalidInput {
        received: u32,
    },
    NoInput {
        num_tick: u32,
    },
    InvalidInputLength {
        received: u32,
        expected: u32,
        initial_tick: u32,
        end_tick: u32,
    },
    USizeToU32Conversion {},
}

impl fmt::Display for SimulationError {
    #[allow(clippy::uninlined_format_args)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidInput { received } => {
                write!(f, "Input invalid: received {}", received)
            }
            Self::NoInput { num_tick } => {
                write!(f, "No input at tick: {}", num_tick)
            }
            Self::InvalidInputLength {
                received,
                expected,
                initial_tick,
                end_tick,
            } => {
                write!(
                    f,
                    "Invalid input length: received {} expected {} initial tick {} end tick {}",
                    received, expected, initial_tick, end_tick
                )
            }
            Self::USizeToU32Conversion {} => {
                write!(f, "Failed to convert USizeToU32")
            }
        }
    }
}
