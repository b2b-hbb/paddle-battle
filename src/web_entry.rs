extern crate alloc;

use alloc::vec::Vec;
use console_error_panic_hook::set_once;
use wasm_bindgen::prelude::*;
use crate::consts;
use crate::world::GameState;

#[wasm_bindgen]
pub struct WasmState {
    inner: GameState,
}

#[wasm_bindgen]
impl WasmState {
    #[wasm_bindgen(constructor)]
    #[allow(
        clippy::new_without_default,
        clippy::must_use_candidate,
        clippy::missing_const_for_fn
    )]
    pub fn new() -> Self {
        Self {
            inner: GameState::new(),
        }
    }

    /// # Panics
    ///
    /// Will panic if there is an error in the ticks
    #[wasm_bindgen]
    #[allow(clippy::needless_pass_by_value)]
    pub fn tick(&mut self, num_ticks: u32, input: Vec<u32>) {
        // TODO: only set_once in debug mode
        set_once();

        match self.inner.tick(num_ticks, &input) {
            Ok(()) => (),
            #[allow(clippy::uninlined_format_args)]
            Err(e) => panic!("Error processing ticks: {}", e),
        }
    }

    #[wasm_bindgen]
    #[allow(clippy::must_use_candidate)]
    pub fn to_cbor(&self) -> Vec<u8> {
        self.inner.to_serialized_state()
    }

    #[wasm_bindgen]
    pub fn tick_and_return_state(&mut self, ticks: u32, input: Vec<u32>) -> Vec<u8> {
        self.tick(ticks, input);
        self.to_cbor()
    }

    #[wasm_bindgen]
    #[allow(clippy::must_use_candidate, clippy::missing_const_for_fn)]
    pub fn get_max_x(&self) -> u32 {
        consts::WORLD_MAX_X
    }

    #[wasm_bindgen]
    #[allow(clippy::must_use_candidate, clippy::missing_const_for_fn)]
    pub fn get_max_y(&self) -> u32 {
        consts::WORLD_MAX_Y
    }

    #[wasm_bindgen]
    #[allow(clippy::must_use_candidate, clippy::missing_const_for_fn)]
    pub fn ticks_per_input(&self) -> u32 {
        consts::TICKS_PER_INPUT
    }

    #[wasm_bindgen]
    #[allow(clippy::must_use_candidate, clippy::missing_const_for_fn)]
    pub fn tick_input_api_chunk_size(&self) -> u32 {
        consts::TICK_INPUT_API_CHUNK_SIZE
    }
}
