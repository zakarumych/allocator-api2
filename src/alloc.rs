//! Re-exports from [`alloc`].
//! Requires "alloc" feature.

#[cfg(not(feature = "nightly"))]
pub mod alloc;

#[cfg(feature = "nightly")]
#[doc(inline)]
pub use real_alloc::alloc;
