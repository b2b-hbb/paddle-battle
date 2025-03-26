extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{
    alloy_primitives::{B256, U256},
    alloy_sol_types::sol,
    crypto::keccak,
    evm,
    prelude::*,
    storage::StorageFixedBytes,
};

use crate::world::GameState;

// Define some persistent storage using the Solidity ABI.
// `PaddleBattle` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct PaddleBattle {
        bytes32 game_state_hash;
    }
}

sol! {
    event GameStateEvent(bytes32 game_state_hash, uint256 left_raft_health, uint256 right_raft_health, uint256 left_projectile_count, uint256 right_projectile_count);
}

impl GameState {
    pub fn hash(&self) -> B256 {
        let serialized_game_state = self.to_serialized_state();
        let hash = keccak(&serialized_game_state);
        hash
    }
}

/// Declare that `PaddleBattle` is a contract with the following external methods.
#[public]
impl PaddleBattle {
    pub fn game_state_hash(&self) -> B256 {
        self.game_state_hash.get()
    }

    pub fn tick(&mut self, num_ticks: u32, inputs: Vec<u32>) {
        let mut curr_game_state = GameState::new();
        _tick(
            num_ticks,
            &inputs,
            &mut curr_game_state,
            &mut self.game_state_hash,
        );
    }

    pub fn load_and_tick(&mut self, num_ticks: u32, inputs: Vec<u32>, serialized_state: String) {
        // TODO: more efficient encoding/decoding of game state since this bloats the contract size
        // let mut curr_game_state = GameState::from_serialized_state(serialized_state);
        let mut curr_game_state = GameState::new();
        let prev_hash = self.game_state_hash();
        if prev_hash != curr_game_state.hash() {
            panic!("Previous game state hash mismatch");
        }

        _tick(
            num_ticks,
            &inputs,
            &mut curr_game_state,
            &mut self.game_state_hash,
        );
    }
}

fn _tick(
    num_ticks: u32,
    inputs: &Vec<u32>,
    curr_game_state: &mut GameState,
    game_state_hash_storage: &mut StorageFixedBytes<32>,
) {
    if !validate_inputs(&inputs) {
        panic!("invalid inputs");
    }

    curr_game_state
        .tick(num_ticks, &inputs)
        .unwrap_or_else(|e| panic!("SimulationError: {:?}", e));

    let new_hash = curr_game_state.hash();
    game_state_hash_storage.set(new_hash);

    evm::log(GameStateEvent {
        game_state_hash: new_hash,
        left_raft_health: U256::from(curr_game_state.raft_left.curr_health),
        right_raft_health: U256::from(curr_game_state.raft_right.curr_health),
        left_projectile_count: U256::from(curr_game_state.left_projectiles.len()),
        right_projectile_count: U256::from(curr_game_state.right_projectiles.len()),
    });
}

fn validate_inputs(inputs: &Vec<u32>) -> bool {
    // TODO: validate inputs. this could be a sequencer's sig or countersignatures by the players involved
    true
}
