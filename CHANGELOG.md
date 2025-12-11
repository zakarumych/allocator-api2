# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 11.12.2025

### Added

- Implementation of `Allocator for &mut A` as it was added to `core`
- Implementation `Default for vec::IntoIter`
- Suffixed methods to `SliceExt`


### Changed

- Use `core::error::Error` under "fresh-rust" feature even without "std" feature.
- `Serialize for Box<T, A>` is relaxed to accept `T: ?Sized`
- `serde_core` crate is used instead of `serde` to remove dependency on `serde_derive` when some other crate enables "derive" feture in `serde`
