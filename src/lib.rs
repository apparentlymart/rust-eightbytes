//! Implements "SIMD-like" operations over vectors of eight `u8` values
//! represented internally as single `u64` values, using bit-fidding tricks.
//!
//! These techniques are sometimes described as "SWAR": SIMD within a register.
//!
//! The API is designed to loosely follow the conventions of the Rust standard
//! library's portable SIMD module, but only to a limited extent. There is no
//! explicit interop between the two.
#![cfg_attr(not(test), no_std)]

#[path = "mask8x8.rs"]
mod maskmod;
#[path = "u8x8.rs"]
mod vecmod;

pub use maskmod::*;
pub use vecmod::*;

#[cfg(test)]
mod u8x8_tests;

#[cfg(test)]
mod mask8x8_tests;
