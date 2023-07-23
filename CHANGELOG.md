# Changelog
All notable changes to `parsenic` will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [0.2.0] - Unreleased
### Added
 - `error` module with error types
 - `result` module with `Result` type aliases

### Changed
 - Bounds of `Int` and `UInt`
 - Now depends on `traitful` for extension traits

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
