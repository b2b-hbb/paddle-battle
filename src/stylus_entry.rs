extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, alloy_sol_types::sol, evm, prelude::*};

use crate::world::GameState;

// Define some persistent storage using the Solidity ABI.
// `PaddleBattle` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct PaddleBattle {
        uint256 number;
        // GameState game_state;
    }
}

sol! {
    event GameStateEvent(uint256 left_raft_health, uint256 right_raft_health, uint256 left_projectile_count, uint256 right_projectile_count);
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
    pub fn tick(&mut self, num_ticks: U256, inputs: Vec<U256>) {
        let mut state = GameState::new();
        
        state.tick(num_ticks, &inputs).unwrap_or_else(|e| panic!("SimulationError: {:?}", e));

        self.increment();

        evm::log(GameStateEvent{
            left_raft_health: U256::from(state.raft_left.curr_health),
            right_raft_health: U256::from(state.raft_right.curr_health),
            left_projectile_count: U256::from(state.left_projectiles.len()),
            right_projectile_count: U256::from(state.right_projectiles.len()),
        });
    }
}
