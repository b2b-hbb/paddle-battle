#![warn(clippy::pedantic, clippy::nursery, clippy::all)]

#![cfg_attr(not(feature = "export-abi"), no_std)]

#[cfg(feature = "stylus")]
pub mod stylus_entry;

#[cfg(feature = "web")]
pub mod web_entry;

pub mod consts;

pub mod world;

pub mod errors;

pub mod physics;

pub mod paddle;
