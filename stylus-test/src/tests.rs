#[tokio::test]
async fn integration_test() {
    use crate::abi::PaddleBattle;
    use crate::common::setup;
    use crate::{GameInput, TICKS_PER_INPUT, TICK_INPUT_API_CHUNK_SIZE};
    use alloy::primitives::Log;
    use alloy::primitives::U256;
    use alloy::{hex::FromHex, primitives::B256};
    use alloy::{
        network::EthereumWallet, providers::ProviderBuilder, signers::local::PrivateKeySigner,
    };
    use alloy::{rpc::types::TransactionReceipt, sol_types::SolEvent};
    use std::path::PathBuf;
    let private_key = "0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659";
    let endpoint = "http://localhost:8547";

    let signer: PrivateKeySigner = private_key.parse().expect("should parse private key");
    // let addr = signer.address();
    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(endpoint.parse().unwrap());

    let manifest_dir = PathBuf::from("..");
    let address = setup(private_key, endpoint, &manifest_dir).await.unwrap();

    let contract = PaddleBattle::new(address, provider.clone());

    // Create input array similar to game loop
    let mut input_codes: Vec<u32> = Vec::new();

    let num_ticks = 1000;

    // Add input codes based on simulated key presses
    // For testing, let's simulate some movement
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
    // should be the number of ticks divided by the number of ticks per input
    let inputs_needed = num_ticks / TICKS_PER_INPUT;

    let mut final_inputs: Vec<u32> = Vec::new();
    for _ in 0..inputs_needed {
        final_inputs.extend(&input_codes);
    }

    let prev_game_state_hash = contract.gameStateHash().call().await.unwrap();

    assert_eq!(
        prev_game_state_hash._0,
        B256::from_hex("0x0000000000000000000000000000000000000000000000000000000000000000")
            .unwrap()
    );

    let pending_tx = contract
        .tick(num_ticks, final_inputs.clone())
        .send()
        .await
        .expect("failed to send tx");

    let receipt = pending_tx
        .get_receipt()
        .await
        .expect("failed to get receipt");

    println!("submitted tx with inputs: {:?}", final_inputs.clone());
    assert_eq!(inputs_needed, 200);

    println!("Transaction gas used: {:?}", receipt.gas_used);

    pub fn decoded_log<E: SolEvent>(receipt: &TransactionReceipt) -> Option<Log<E>> {
        receipt
            .inner
            .logs()
            .iter()
            .find_map(|log| E::decode_log(&log.inner, false).ok())
    }

    let log = decoded_log::<PaddleBattle::GameStateEvent>(&receipt).expect("failed to decode log");
    println!(
        "left raft health: {:?}\nright raft health: {:?}\nleft projectile count: {:?}\nright projectile count: {:?}\ngame state hash: {:?}",
        log.leftRaftHealth, log.rightRaftHealth, log.leftProjectileCount, log.rightProjectileCount, log.gameStateHash
    );

    assert_eq!(log.leftRaftHealth, U256::from(10_000));
    assert_eq!(log.rightRaftHealth, U256::from(9_500));
    assert_eq!(log.leftProjectileCount, U256::from(45));
    assert_eq!(log.rightProjectileCount, U256::from(52));

    assert_eq!(receipt.gas_used, 1_311_943);

    let post_game_state_hash = contract.gameStateHash().call().await.unwrap();
    let expected_post_game_state_hash = B256::from_hex("0x400b4863233cfe96227b2553e1cee0591bbf7c763c454e131231af391bc3f413").unwrap();

    assert_eq!(post_game_state_hash._0, log.gameStateHash);
    assert_eq!(post_game_state_hash._0, expected_post_game_state_hash);

    // Run the same game again locally to get a copy of the game state
    let local_game_state = crate::paddle::simulate_game_state(num_ticks, &final_inputs).unwrap();
    
    // Verify the local game state matches the on-chain state    
    assert_eq!(local_game_state.hash(), expected_post_game_state_hash);

    // Create array of input arrays for multiple ticks
    // should be the number of ticks divided by the number of ticks per input

    let serialized_game_state = local_game_state.to_serialized_state();
    assert_eq!(serialized_game_state.len(), 3658);

    /*

    let pending_tx2 = contract
        .loadAndTick(num_ticks, final_inputs.clone(), serialized_game_state.into())
        .send()
        .await
        .expect("failed to send tx");

    let receipt2 = pending_tx2
        .get_receipt()
        .await
        .expect("failed to get receipt");

    println!("submitted tx with inputs: {:?}", final_inputs.clone());
    println!("Transaction gas used: {:?}", receipt2.gas_used);

    let log2 = decoded_log::<PaddleBattle::GameStateEvent>(&receipt2).expect("failed to decode log");
    println!(
        "left raft health: {:?}\nright raft health: {:?}\nleft projectile count: {:?}\nright projectile count: {:?}\ngame state hash: {:?}",
        log2.leftRaftHealth, log2.rightRaftHealth, log2.leftProjectileCount, log2.rightProjectileCount, log2.gameStateHash
    );

    assert_eq!(log2.leftRaftHealth, U256::from(10_000));
    assert_eq!(log2.rightRaftHealth, U256::from(9_500));
    assert_eq!(log2.leftProjectileCount, U256::from(47));
    assert_eq!(log2.rightProjectileCount, U256::from(72));

    assert_eq!(receipt2.gas_used, 2_346_623);

    let expected_post_game_state_hash2 = B256::from_hex("0x9512bd96506ebde6ed7ac5e3a638c92c7f8453779a54e8660a7b4ee9b3e20374").unwrap();
    let post_game_state_hash2 = contract.gameStateHash().call().await.unwrap();
    assert_eq!(post_game_state_hash2._0, log2.gameStateHash);
    assert_eq!(post_game_state_hash2._0, expected_post_game_state_hash2);

    */

    // TODO: now execute a test from the UI over here by loading the inputs and then calling the tick function
}
