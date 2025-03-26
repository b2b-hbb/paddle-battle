extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, alloy_primitives::B256, alloy_sol_types::sol, crypto::keccak, evm, prelude::*};

use crate::world::GameState;

// Define some persistent storage using the Solidity ABI.
// `PaddleBattle` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct PaddleBattle {
        uint256 number;
        bytes32 game_state_hash;
    }
}

sol! {
    event GameStateEvent(bytes32 game_state_hash, uint256 left_raft_health, uint256 right_raft_health, uint256 left_projectile_count, uint256 right_projectile_count);
}

impl GameState {
    // TODO: assess abi encoding of game state. current approach bloats contract size too much
    // pub fn from_calldata(calldata: String) -> Self {
    //     serde_json::from_str(&calldata).expect("failed to deserialize game state")
    // }

    pub fn hash(&self) -> B256 {
        let serialized_game_state = serde_json::to_string(&self).expect("Failed to serialize game state");
        let hash = keccak(&serialized_game_state);
        hash
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

    pub fn game_state_hash(&self) -> B256 {
        self.game_state_hash.get()
    }

    /// Increments `number` and updates its value in storage.
    pub fn increment(&mut self) {
        let number = self.number.get();
        self.set_number(number + U256::from(1));
    }

    #[allow(
        clippy::must_use_candidate,
        clippy::needless_pass_by_value,
        clippy::uninlined_format_args
    )]
    pub fn tick(&mut self, num_ticks: u32, inputs: Vec<u32>) {
        let mut curr_game_state = GameState::new();
        
        curr_game_state.tick(num_ticks, &inputs).unwrap_or_else(|e| panic!("SimulationError: {:?}", e));

        self.increment();

        let new_hash = curr_game_state.hash();
        self.game_state_hash.set(new_hash);

        evm::log(GameStateEvent{
            game_state_hash: new_hash,
            left_raft_health: U256::from(curr_game_state.raft_left.curr_health),
            right_raft_health: U256::from(curr_game_state.raft_right.curr_health),
            left_projectile_count: U256::from(curr_game_state.left_projectiles.len()),
            right_projectile_count: U256::from(curr_game_state.right_projectiles.len()),
        });
    }

    pub fn load_and_tick(
        &mut self,
        num_ticks: u32,
        inputs: Vec<u32>,
        calldata: String
    ) {
        
        // let mut curr_game_state = GameState::from_calldata(calldata);
        let mut curr_game_state = GameState::new();

        let prev_hash = self.game_state_hash();

        if prev_hash != curr_game_state.hash() {
            panic!("Game state hash mismatch");
        }
        
        curr_game_state.tick(num_ticks, &inputs).unwrap_or_else(|e| panic!("SimulationError: {:?}", e));

        let new_hash = curr_game_state.hash();
        self.game_state_hash.set(new_hash);
        
        evm::log(GameStateEvent{
            game_state_hash: new_hash,
            left_raft_health: U256::from(curr_game_state.raft_left.curr_health),
            right_raft_health: U256::from(curr_game_state.raft_right.curr_health),
            left_projectile_count: U256::from(curr_game_state.left_projectiles.len()),
            right_projectile_count: U256::from(curr_game_state.right_projectiles.len()),
        });
    }
}
