use crate::game::GameInput;
use crate::game::TICKS_PER_INPUT;
use crate::game::TICK_INPUT_API_CHUNK_SIZE;
use paddle_battle::world::GameState;
use paddle_battle::errors::Result;

pub fn simulate_game_state(ticks: u32, inputs: &[u32]) -> Result<GameState> {
    let mut game_state = GameState::new();
    game_state.tick(ticks, &inputs.to_vec())?;
    Ok(game_state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_simulation() {
        let mut input_codes: Vec<u32> = Vec::new();

        // Add input codes based on simulated key presses
        input_codes.push(GameInput::MoveRightRaftLeft.to_u32());
        input_codes.push(GameInput::MoveLeftRaftRight.to_u32());
        input_codes.push(GameInput::MoveRightRaftLeft.to_u32());
        input_codes.push(GameInput::MoveLeftRaftRight.to_u32());
        input_codes.push(GameInput::MoveRightRaftLeft.to_u32());
        input_codes.push(GameInput::MoveLeftRaftRight.to_u32());
        input_codes.push(GameInput::MoveUpRaftLeft.to_u32());

        // Pad the input array to match TICK_INPUT_API_CHUNK_SIZE
        while input_codes.len() < TICK_INPUT_API_CHUNK_SIZE as usize {
            input_codes.push(GameInput::NoOp.to_u32());
        }

        // Create array of input arrays for multiple ticks
        let num_ticks = 1000;
        let inputs_needed = num_ticks / TICKS_PER_INPUT;
        let mut final_inputs: Vec<u32> = Vec::new();
        for _ in 0..inputs_needed {
            final_inputs.extend(&input_codes);
        }

        let game_state = simulate_game_state(num_ticks, &final_inputs).unwrap();
        
        // Verify the game state matches the on-chain state
        assert_eq!(game_state.raft_left.curr_health, 10_000);
        assert_eq!(game_state.raft_right.curr_health, 9_500);
        assert_eq!(game_state.left_projectiles.len(), 45);
        assert_eq!(game_state.right_projectiles.len(), 52);
    }
} 