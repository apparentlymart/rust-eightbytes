# `eight_bytes` for Rust

This library implements types `u8x8` and `mask8x8` which both represent a
vector of eight values packed into a `u64`, where the first is a vector of `u8`
and the second is a vector of `bool`.

The methods of these types can perform limited SIMD-like operations on those
vectors without using native SIMD instructions, by using various bit
manipulation tricks.
