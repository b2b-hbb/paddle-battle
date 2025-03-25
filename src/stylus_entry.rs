extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, prelude::*};

use crate::world::GameState;

// Define some persistent storage using the Solidity ABI.
// `PaddleBattle` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct PaddleBattle {
        uint256 number;
    }
}

/// Declare that `PaddleBattle` is a contract with the following external methods.
#[public]
impl PaddleBattle {
    /// Gets the number from storage.
    pub fn number(&self) -> U256 {
        self.number.get()
    }

    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) {
        self.number.set(new_number);
    }

    /// Increments `number` and updates its value in storage.
    pub fn increment(&mut self) {
        let number = self.number.get();
        self.set_number(number + U256::from(1));
    }

    /// # Panics
    ///
    /// Will panic if there is an error in gamestate ticks
    #[allow(
        clippy::must_use_candidate,
        clippy::needless_pass_by_value,
        clippy::uninlined_format_args
    )]
    pub fn tick(&mut self, num_ticks: u32, inputs: Vec<u32>) -> String {
        let mut state = GameState::new();
        match state.tick(num_ticks, &inputs) {
            Ok(()) => serde_json::to_string(&state).unwrap_or_else(|_| "{}".to_string()),
            Err(e) => panic!("err {}", e),
        }
    }
}
