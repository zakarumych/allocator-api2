#![cfg_attr(feature = "nightly", feature(allocator_api))]

use std::alloc::Layout;

use allocator_api2::{alloc::Allocator, vec::Vec};

#[macro_export]
macro_rules! make_test {
    ($($test_name:ident($($arg:expr),* $(,)?)),* $(,)?) => {
        $(
            #[test]
            fn $test_name() {
                $crate::$test_name($($arg),*);
            }
        )*
    };
}

pub fn test_allocate_layout<A: Allocator>(alloc: A, layout: Layout) {
    if let Ok(ptr) = alloc.allocate(layout) {
        unsafe { alloc.deallocate(ptr.cast(), layout) }
    }
}

pub fn test_sizes<A: Allocator>(alloc: A) {
    test_allocate_layout(&alloc, Layout::new::<u8>());
    test_allocate_layout(&alloc, Layout::new::<u16>());
    test_allocate_layout(&alloc, Layout::new::<u32>());
    test_allocate_layout(&alloc, Layout::new::<u64>());
    test_allocate_layout(&alloc, Layout::new::<[u8; 17]>());
    test_allocate_layout(&alloc, Layout::new::<[u8; 67]>());
    test_allocate_layout(&alloc, Layout::new::<[u8; 129]>());
    test_allocate_layout(&alloc, Layout::new::<[u8; 654]>());
    test_allocate_layout(&alloc, Layout::new::<[u8; 2345]>());
    test_allocate_layout(&alloc, Layout::new::<[u8; 32578]>());
    test_allocate_layout(&alloc, Layout::new::<[u8; 8603461]>());
}

pub fn test_vec<A: Allocator>(alloc: A) {
    let mut vec = Vec::<u8, A>::new_in(alloc);

    vec.push(1);
    vec.push(2);
    vec.shrink_to_fit();
    vec.push(3);

    vec.resize(10, 0xba);
    vec.shrink_to_fit();

    vec.resize(12467, 0xfe);
    drop(vec);
}
