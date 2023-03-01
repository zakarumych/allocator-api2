//!
//! allocator-api2 crate.
//!
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(not(feature = "nightly"))]
mod core2;

#[cfg(not(feature = "nightly"))]
#[cfg(feature = "alloc")]
mod alloc2;

#[cfg(not(feature = "nightly"))]
#[cfg(not(feature = "alloc"))]
#[doc(inline)]
pub use self::core2::*;

#[cfg(not(feature = "nightly"))]
#[cfg(feature = "alloc")]
#[doc(inline)]
pub use self::alloc2::*;

#[cfg(feature = "nightly")]
#[cfg(not(feature = "alloc"))]
#[doc(inline)]
pub use core::alloc::*;

#[cfg(feature = "nightly")]
#[cfg(feature = "alloc")]
#[doc(inline)]
pub use alloc::alloc::*;

#[cfg(feature = "nightly")]
#[cfg(feature = "alloc")]
#[doc(inline)]
pub use alloc::{boxed, vec};

#[cfg(feature = "alloc")]
#[cfg(not(feature = "nightly"))]
#[macro_export]
macro_rules! vec {
    ($($t:tt)*) => {
        alloc::vec!( $($t)* )
    };
}
