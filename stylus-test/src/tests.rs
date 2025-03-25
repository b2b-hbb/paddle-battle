use std::path::PathBuf;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::{
    network::EthereumWallet, providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use crate::common::setup;
use crate::{GameInput, TICKS_PER_INPUT, TICK_INPUT_API_CHUNK_SIZE};
use crate::abi::PaddleBattle;

#[tokio::test]
async fn integration_test() {
    let private_key = "0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659";
    let endpoint = "http://localhost:8547";

    let signer: PrivateKeySigner = private_key.parse().expect("should parse private key");
    let addr = signer.address();
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

    let tx_hash = contract
        .tick(num_ticks, final_inputs.clone())
        .send()
        .await
        .expect("failed to send tx")
        .watch()
        .await
        .expect("failed to submit tx");

    println!("submitted tx with inputs: {:?}", final_inputs.clone());
    
    // Fetch and print transaction receipt
    let receipt = provider.get_transaction_receipt(tx_hash).await.expect("failed to get receipt").expect("receipt is none");
    println!("Transaction gas used: {:?}", receipt.gas_used);

    assert_eq!(receipt.gas_used, 783_596);
} 