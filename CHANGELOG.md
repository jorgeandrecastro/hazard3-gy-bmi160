# Changelog

All notable changes to this project are documented in this file.

This changelog follows [Keep a Changelog](https://keepachangelog.com/), and
this project follows [Semantic Versioning](https://semver.org/).

## [0.4.0] - 2026-11-25

### Changed

- Migrated the package to the `hazard3-gy-bmi160` crate name.
- Added selectable Hazard3 (default) and Embassy time and synchronization
  backends.
- Added `init_with_delay` for an application-provided async delay, while `init`
  uses the selected time backend.
- Rewrote project documentation and examples in English.

## [0.3.0] - 2026-05-02

### Changed

- Replaced dependency version ranges with explicit versions for more
  reproducible builds.

## [0.2.1] - 2026-04-26

### Changed

- Expanded supported time and synchronization dependency versions.
- Added a minimal example using `set_address`, `read_gyro`, and `read_accel`.

## [0.2.0] - 2026-02-15

### Added

- Added `#![forbid(unsafe_code)]`.
- Added an OLED example with gyro and accelerometer readings.
- Added a `signals` module for inter-task communication.

### Changed

- Refactored for zero allocation and bare-metal use.

## [0.1.0] - 2025-12-01

### Added

- Initial async BMI160 sensor driver.
- Non-blocking I2C access through `embedded-hal-async`.
- Raw `i16` sample data and shared I2C bus support.
