#![deny(unsafe_op_in_unsafe_fn)]

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
#[inline(always)]
#[cfg(debug_assertions)]
unsafe fn assume(v: bool) {
    if !v {
        core::unreachable!()
    }
}

#[inline(always)]
#[cfg(not(debug_assertions))]
unsafe fn assume(v: bool) {
    if !v {
        core::hint::unreachable_unchecked()
    }
}

#[inline(always)]
fn addr<T>(x: *const T) -> usize {
    unsafe { core::mem::transmute(x) }
}

#[inline(always)]
fn invalid_mut<T>(addr: usize) -> *mut T {
    unsafe { core::mem::transmute(addr) }
}
