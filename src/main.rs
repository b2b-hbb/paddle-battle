// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]

#[cfg(feature = "export-abi")]
fn main() {
    paddle_battle::stylus_entry::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}
