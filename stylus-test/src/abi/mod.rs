#![allow(dead_code)]
use alloy::sol;

sol!(
    #[sol(rpc)]
   contract PaddleBattle {
        function gameStateHash() external view returns (bytes32);

        function tick(uint32 num_ticks, uint32[] calldata inputs) external;

        event GameStateEvent(bytes32 gameStateHash, uint256 leftRaftHealth, uint256 rightRaftHealth, uint256 leftProjectileCount, uint256 rightProjectileCount);
   }
);