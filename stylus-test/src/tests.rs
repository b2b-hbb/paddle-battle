
#[tokio::test]
async fn integration_test() {
    use std::path::PathBuf;
    use alloy::primitives::U256;
    use alloy::primitives::Log;
    use alloy::{rpc::types::TransactionReceipt, sol_types::SolEvent};    
    use alloy::{
        network::EthereumWallet, providers::ProviderBuilder,
        signers::local::PrivateKeySigner,
    };
    use crate::common::setup;
    use crate::{GameInput, TICKS_PER_INPUT, TICK_INPUT_API_CHUNK_SIZE};
    use crate::abi::PaddleBattle;
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

    let prev_num = contract.number().call().await;
    assert_eq!(prev_num.unwrap()._0, U256::from(0));

    let _ = contract
        .increment()
        .send()
        .await
        .expect("failed to send tx")
        .watch()
        .await
        .expect("failed to submit tx");

    let new_num = contract.number().call().await;
    assert_eq!(new_num.unwrap()._0, U256::from(1));


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

    let pending_tx = contract
        .tick(num_ticks, final_inputs.clone())
        .send()
        .await
        .expect("failed to send tx");

    let receipt = pending_tx.get_receipt().await.expect("failed to get receipt");


    println!("submitted tx with inputs: {:?}", final_inputs.clone());
    println!("Transaction gas used: {:?}", receipt.gas_used);

    pub fn decoded_log<E: SolEvent>(receipt: &TransactionReceipt) -> Option<Log<E>> {
        receipt.inner.logs().iter().find_map(|log| E::decode_log(&log.inner, false).ok())
    }

    let log = decoded_log::<PaddleBattle::GameStateEvent>(&receipt).expect("failed to decode log");
    println!(
        "left raft health: {:?}\nright raft health: {:?}\nleft projectile count: {:?}\nright projectile count: {:?}",
        log.leftRaftHealth, log.rightRaftHealth, log.leftProjectileCount, log.rightProjectileCount
    );

    assert_eq!(log.leftRaftHealth, U256::from(10_000));
    assert_eq!(log.rightRaftHealth, U256::from(9_500));
    assert_eq!(log.leftProjectileCount, U256::from(45));
    assert_eq!(log.rightProjectileCount, U256::from(0));

    assert_eq!(receipt.gas_used, 756_148);
} 