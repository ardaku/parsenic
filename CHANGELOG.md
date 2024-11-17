# Changelog
All notable changes to `parsenic` will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [0.2.1] - 2024-11-16

### Added

 - `le::Read::f32()`
 - `le::Read::f64()`
 - `le::Write::f32()`
 - `le::Write::f64()`
 - `be::Read::f32()`
 - `be::Read::f64()`
 - `be::Write::f32()`
 - `be::Write::f64()`

## [0.2.0] - 2024-11-05

### Added
 - `error` module with error types
 - `result` module with `Result` type aliases
 - `unstable-error` feature implementing `Error` for error types
 - `Empty` reader and `empty()` constructor
 - `Purge` writer and `purge()` constructor
 - `class` module
 - `Read` and `Write` extension traits on `Reader` and `Writer`

### Changed
 - Bounds of `Int` and `UInt`
 - Now depends on `traitful` for extension traits
 - `io` module is behind a feature flag
 - `UInt` and `Int` are now in `class` module

### Fixed
 - Broken links in README and docs
 - Parsing `u8()` not advancing reader

## [0.1.0] - 2023-07-16

### Added
 - `be` module
 - `le` module
 - `Reader` struct
 - `Writer` struct
 - `Int` trait
 - `UInt` trait

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://github.com/AldaronLau/semver/blob/stable/README.md
