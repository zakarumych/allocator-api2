pub mod alloc;

#[cfg(feature = "alloc")]
pub mod boxed;

#[cfg(feature = "alloc")]
mod raw_vec;

#[cfg(feature = "alloc")]
pub mod vec;

#[cfg(feature = "alloc")]
mod macros;

#[track_caller]
#[cfg(debug_assertions)]
unsafe fn assume(v: bool) {
    if !v {
        core::unreachable!()
    }
}

#[cfg(not(debug_assertions))]
unsafe fn assume(v: bool) {
    if !v {
        core::hint::unreachable_unchecked()
    }
}
