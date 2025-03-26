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
    assert_eq!(log.rightProjectileCount, U256::from(0));

    assert_eq!(receipt.gas_used, 790_754);

    let post_game_state_hash = contract.gameStateHash().call().await.unwrap();

    assert_eq!(post_game_state_hash._0, log.gameStateHash);
    assert_eq!(
        post_game_state_hash._0,
        B256::from_hex("0xaf5a095d6a5f3d5bb2ae78b550bf07c374cdeb7ffdda9fef1072ab75aea3d263")
            .unwrap()
    );

    // TODO: now execute a test from the UI over here by loading the inputs and then calling the tick function
}
