//! Module containing extra tests, for past issues. These are not bulk copied from std.

use crate::boxed::Box;
use crate::vec::Vec;

/// This used to cause UB.
#[test]
fn issue_21() {
    let b = Box::new(3);
    Box::into_inner(b);
}

/// This used to cause UB, instead of panicing.
#[test]
#[should_panic]
fn issue_47() {
    let mut v: Vec<u8> = Vec::new();
    v.push(1);

    let idx = v.capacity().saturating_mul(4).saturating_add(1);
    v.insert(idx, 2);
}

/// This should be a reproducer for the specific issue mentioned in issue 29
/// but doesn't seem to trigger miri.
#[test]
fn issue_29() {
    let mut v: Vec<u8> = crate::vec![1];
    let _splice = v.splice(0..0, [2]);
}

/// This used to get stuck in a loop with almost `usize::MAX` iterations as witnessed by the output (in release mode) of `18446744073709551611` (instead of `5`).
#[test]
fn zst_vec_rev() {
    #[derive(Clone, Copy)]
    struct Zst;
    let x = crate::vec![Zst; 5].into_iter().rev().count();
    assert_eq!(x, 5);
}
