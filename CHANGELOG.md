# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- `async` capabilities via `maybe_async`. Enable the `async` feature (and disable `sync`) to use `async` versions of each function.
- example project for ESP32-S3 which uses the `async` feature

### Removed

- Removed `BuslessMpr`, use [BusSharing](https://docs.rs/embedded-hal/latest/embedded_hal/i2c/index.html#bus-sharing) instead.

## [0.4] - 2024-04-25

### Changed
- Update to `embedded_hal` 1.0
