//!
//! allocator-api2 crate.
//!
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc as real_alloc;

pub mod core;

#[cfg(feature = "alloc")]
pub mod alloc;

#[track_caller]
unsafe fn assume(v: bool) {
    if !v {
        unreachable_unchecked()
    }
}

#[cfg(debug_assertions)]
#[track_caller]
unsafe fn unreachable_unchecked() {
    unreachable!()
}

#[cfg(not(debug_assertions))]
#[cfg_attr(miri, track_caller)]
unsafe fn unreachable_unchecked() {
    core::hint::unreachable_unchecked()
}
