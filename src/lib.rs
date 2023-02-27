//!
//! allocator-api2 crate.
//!
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc as real_alloc;

pub mod core;

#[cfg(feature = "alloc")]
pub mod alloc;
