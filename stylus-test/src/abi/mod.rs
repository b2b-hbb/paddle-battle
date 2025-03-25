#![allow(dead_code)]
use alloy::sol;

sol!(
    #[sol(rpc)]
   contract PaddleBattle {
        // function approve(address to, uint256 tokenId) external;
        // #[derive(Debug)]
        // function balanceOf(address owner) external view returns (uint256 balance);
        // #[derive(Debug)]
        // function getApproved(uint256 tokenId) external view returns (address approved);
        // #[derive(Debug)]
        // function isApprovedForAll(address owner, address operator) external view returns (bool approved);
        // #[derive(Debug)]
        // function ownerOf(uint256 tokenId) external view returns (address ownerOf);
        // function safeTransferFrom(address from, address to, uint256 tokenId) external;
        // function safeTransferFrom(address from, address to, uint256 tokenId, bytes calldata data) external;
        // function setApprovalForAll(address operator, bool approved) external;
        // function totalSupply() external view returns (uint256 totalSupply);
        // function transferFrom(address from, address to, uint256 tokenId) external;
        // function safeMint(address to, uint256 tokenId, bytes calldata data) external;
        // function mint(address to, uint256 tokenId) external;
        // function burn(uint256 tokenId) external;

        function number() external view returns (uint256);
        function setNumber(uint256 newNumber) external;
        function increment() external;
        function tick(uint32 num_ticks, uint32[] calldata inputs) external;

        event GameStateEvent(uint256 leftRaftHealth, uint256 rightRaftHealth, uint256 leftProjectileCount, uint256 rightProjectileCount);
   }
);