//!
//! allocator_api2 crate.
//!
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc as real_alloc;

#[cfg(not(feature = "nightly"))]
mod core;

#[cfg(not(feature = "nightly"))]
mod alloc;

#[cfg(not(feature = "nightly"))]
#[doc(inline)]
pub use self::core::*;

#[cfg(feature = "nightly")]
#[doc(inline)]
pub use core::alloc::*;

#[cfg(not(feature = "nightly"))]
#[cfg(feature = "alloc")]
#[doc(inline)]
pub use self::alloc::*;

#[cfg(feature = "nightly")]
#[cfg(feature = "alloc")]
#[doc(inline)]
pub use core::alloc::*;
