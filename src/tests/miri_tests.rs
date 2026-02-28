//! Tests copied from miris test suite

use crate::vec::Vec;

// Gather all references from a mutable iterator and make sure Miri notices if
// using them is dangerous.
fn test_all_refs<'a, T: 'a>(dummy: &mut T, iter: impl Iterator<Item = &'a mut T>) {
    // Gather all those references.
    let mut refs: Vec<&mut T> = iter.collect();
    // Use them all. Twice, to be sure we got all interleavings.
    for r in refs.iter_mut() {
        std::mem::swap(dummy, r);
    }
    for r in refs {
        std::mem::swap(dummy, r);
    }
}

#[test]
fn make_vec() {
    let mut v = Vec::with_capacity(4);
    v.push(1);
    v.push(2);
    assert_eq!(v.capacity(), 4);
}

#[test]
fn make_vec_macro() {
    let v = crate::vec![1, 2];
    assert_eq!(v, [1, 2]);
}

#[test]
fn make_vec_macro_repeat() {
    let v = crate::vec![42; 5];
    assert_eq!(v, [42; 5]);
}

#[test]
fn make_vec_macro_repeat_zeroed() {
    let v = crate::vec![0; 7];
    assert_eq!(v, [0; 7]);
}

#[test]
fn vec_into_iter() {
    let x = crate::vec![1, 2, 3, 4]
        .into_iter()
        .map(|x| x * x)
        .fold(0, |x, y| x + y);
    assert_eq!(x, 30);
}

#[test]
fn vec_into_iter_rev() {
    let x = crate::vec![1, 2, 3, 4]
        .into_iter()
        .rev()
        .map(|x| x * x)
        .fold(0, |x, y| x + y);
    assert_eq!(x, 30);
}

#[test]
fn vec_into_iter_rev_zst() {
    for _ in crate::vec![[0u64; 0]; 5].into_iter().rev() {}
    let v = crate::vec![[0u64; 0], [0u64; 0]]
        .into_iter()
        .rev()
        .map(|x| x.len())
        .sum::<usize>();
    assert_eq!(v, 0);
}

#[test]
fn vec_iter_and_mut() {
    let mut v = crate::vec![1, 2, 3, 4];
    for i in v.iter_mut() {
        *i += 1;
    }
    assert_eq!(v.iter().sum::<i32>(), 2 + 3 + 4 + 5);

    test_all_refs(&mut 13, v.iter_mut());
}

#[test]
fn vec_iter_and_mut_rev() {
    let mut v = crate::vec![1, 2, 3, 4];
    for i in v.iter_mut().rev() {
        *i += 1;
    }
    assert_eq!(v.iter().sum::<i32>(), 2 + 3 + 4 + 5);
}

#[test]
fn vec_reallocate() {
    let mut v = crate::vec![1, 2];
    v.push(3);
    v.push(4);
    v.push(5);
    assert_eq!(v.len(), 5);
}

#[test]
fn vec_push_ptr_stable() {
    let mut v = Vec::with_capacity(10);
    v.push(0);
    let v0 = unsafe { &mut *(&mut v[0] as *mut _) }; // laundering the lifetime -- we take care that `v` does not reallocate, so that's okay.
    v.push(1);
    *v0 = *v0;
}

#[test]
fn vec_extend_ptr_stable() {
    let mut v = Vec::with_capacity(10);
    v.push(0);
    let v0 = unsafe { &mut *(&mut v[0] as *mut _) }; // laundering the lifetime -- we take care that `v` does not reallocate, so that's okay.
                                                     // `slice::Iter` (with `T: Copy`) specialization
    v.extend(&[1]);
    *v0 = *v0;
    // `vec::IntoIter` specialization
    v.extend(crate::vec![2]);
    *v0 = *v0;
    // `TrustedLen` specialization
    v.extend(std::iter::once(3));
    *v0 = *v0;
    // base case
    v.extend(std::iter::once(3).filter(|_| true));
    *v0 = *v0;
}

#[test]
fn vec_truncate_ptr_stable() {
    let mut v = crate::vec![0; 10];
    let v0 = unsafe { &mut *(&mut v[0] as *mut _) }; // laundering the lifetime -- we take care that `v` does not reallocate, so that's okay.
    v.truncate(5);
    *v0 = *v0;
}

#[test]
fn push_str_ptr_stable() {
    let mut buf = String::with_capacity(11);
    buf.push_str("hello");
    let hello: &str = unsafe { &*(buf.as_str() as *const _) }; // laundering the lifetime -- we take care that `buf` does not reallocate, so that's okay.
    buf.push_str(" world");
    assert_eq!(format!("{}", hello), "hello");
}

#[test]
fn sort() {
    let mut v = crate::vec![1; 20];
    v.push(0);
    v.sort();
}

#[test]
fn swap() {
    let mut v = crate::vec![1, 2, 3, 4];
    v.swap(2, 2);
}

#[test]
fn swap_remove() {
    let mut a = 0;
    let mut b = 1;
    let mut vec = crate::vec![&mut a, &mut b];

    vec.swap_remove(1);
}

#[test]
fn reverse() {
    #[repr(align(2))]
    #[derive(Debug)]
    struct Foo(u8);

    let mut v: Vec<_> = (0..50).map(Foo).collect();
    v.reverse();
    assert!(v[0].0 == 49);
}

#[test]
fn empty_slice_comparison() {
    // Test interesting empty slice comparison
    // (one is a real pointer, one an integer pointer).
    assert_eq!((200..-5).step_by(1).collect::<Vec<isize>>(), []);
}
