#[cfg(not(feature = "alloc"))]
pub use core::alloc;

#[cfg(feature = "alloc")]
pub use alloc_crate::{alloc, boxed, vec, collections};

/// # Note
/// 
/// This is a no-op on nightly. This macro is a stub, to ensure no build errors when switching
/// between nightly and stable. `std` does the conversion automatically.
/// 
/// # Summary
/// 
/// Allows turning a [`Box<T: Sized, A>`][boxed::Box] into a [`Box<U: ?Sized, A>`][boxed::Box] where `T` can be unsizing-coerced into a `U`.
/// 
/// This is the only way to create an `allocator_api2::boxed::Box` of an unsized type on stable.
///
/// With the standard library's `alloc::boxed::Box`, this is done automatically using the unstable unsize traits, but this crate's Box
/// can't take advantage of that machinery on stable. So, we need to use type inference and the fact that you *can*
/// still coerce the inner pointer of a box to get the compiler to help us unsize it using this macro.
///
/// # Example
///
/// ```
/// use allocator_api2::unsize_box;
/// use allocator_api2::boxed::Box;
/// use core::any::Any;
///
/// let sized_box: Box<u64> = Box::new(0);
/// let unsized_box: Box<dyn Any> = unsize_box!(sized_box);
/// ```
#[macro_export]
#[cfg(feature = "alloc")]
macro_rules! unsize_box {( $boxed:expr $(,)? ) => ({
    $boxed
})}