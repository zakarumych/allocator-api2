//! Re-exports from [`core`].

#[cfg(not(feature = "nightly"))]
pub mod alloc;

#[cfg(feature = "nightly")]
#[doc(inline)]
pub use core::alloc;
