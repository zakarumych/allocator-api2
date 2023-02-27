# allocator-api2

[![crates](https://img.shields.io/crates/v/allocator-api2.svg?style=for-the-badge&label=allocator-api2)](https://crates.io/crates/allocator-api2)
[![docs](https://img.shields.io/badge/docs.rs-allocator--api2-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white)](https://docs.rs/allocator-api2)
[![actions](https://img.shields.io/github/actions/workflow/status/zakarumych/allocator-api2/badge.yml?branch=main&style=for-the-badge)](https://github.com/zakarumych/allocator-api2/actions/workflows/badge.yml)
[![MIT/Apache](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?style=for-the-badge)](COPYING)
![loc](https://img.shields.io/tokei/lines/github/zakarumych/allocator-api2?style=for-the-badge)

This crate mirrors types and traits from Rust's unstable [`allocator_api`]
The intention of this crate is to serve as substitution for actual thing
for libs when build on stable and beta channels.
The target users are library authors who implement allocators or collection types
that use allocators, or anyone else who wants using [`allocator_api`]

The crate should be frequently updated with minor version bump.
When [`allocator_api`] is stable this crate will get version `1.0` and simply
reexport from `core`, `alloc` and possibly `std`.

The code is mostly verbatim copy from rust repository.
Mostly attributes are removed.

[`allocator_api`]: https://doc.rust-lang.org/unstable-book/library-features/allocator-api.html

## License

Licensed under either of

* Apache License, Version 2.0, ([license/APACHE](license/APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([license/MIT](license/MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
