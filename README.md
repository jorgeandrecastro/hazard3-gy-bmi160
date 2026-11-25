# hazard3-gy-bmi160

[![crates.io](https://img.shields.io/crates/v/hazard3-gy-bmi160.svg)](https://crates.io/crates/hazard3-gy-bmi160)
[![docs.rs](https://docs.rs/hazard3-gy-bmi160/badge.svg)](https://docs.rs/hazard3-gy-bmi160)
[![License: GPL-2.0-or-later](https://img.shields.io/badge/license-GPL--2.0--or--later-blue.svg)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.html)

Asynchronous `no_std` I2C driver for the Bosch BMI160 six-axis IMU. It reads raw
accelerometer and gyroscope samples and works with any `embedded-hal-async` I2C
implementation.

The driver integrates with the [Hazard3 async ecosystem](https://github.com/jorgeandrecastro/hazard3-async)
on RP2350 Hazard3 RISC-V and also offers an Embassy backend for ARM/Cortex-M.

## Features

- Async I2C access through `embedded-hal-async`
- Raw signed 16-bit XYZ samples for the accelerometer and gyroscope
- Hazard3 or Embassy async time and synchronization backend
- Optional caller-provided async startup delay
- Global single-consumer signals for the latest accelerometer and gyroscope samples
- `no_std`, allocation-free, and `#![forbid(unsafe_code)]`
- Supports shared I2C buses through any compatible I2C device

## Installation

For RP2350 Hazard3 RISC-V, the default backend uses `hazard3-time` and
`hazard3-sync`:

```toml
[dependencies]
hazard3-gy-bmi160 = "0.4"
```

For ARM/Cortex-M, disable the default backend and select Embassy:

```toml
[dependencies]
hazard3-gy-bmi160 = { version = "0.4", default-features = false, features = ["embassy"] }
```

Select exactly one backend. The application must configure a compatible time
driver and executor. The global signals also require a `critical-section`
implementation. Use matching runtime, HAL, and synchronization crates from the
chosen ecosystem; do not mix the Hazard3 and upstream Embassy time drivers.
See the [Hazard3 ecosystem and migration guide](https://github.com/jorgeandrecastro/hazard3-async)
for the Hazard3 runtime and HAL configuration.

## Quick start

Pass an async I2C device and initialize the sensor with the selected time
backend. This generic example also requires `embedded-hal-async = "1.0"` as a
direct dependency:

```rust
use embedded_hal_async::i2c::I2c;
use hazard3_gy_bmi160::{Bmi160, BmiData};

async fn read_samples<I>(i2c: I) -> Result<(BmiData, BmiData), I::Error>
where
    I: I2c,
{
    let mut imu = Bmi160::new(i2c, 0x68);
    imu.init().await?;

    let accelerometer = imu.read_accel().await?;
    let gyroscope = imu.read_gyro().await?;
    Ok((accelerometer, gyroscope))
}
```

`BmiData` exposes the raw `x`, `y`, and `z` values. Convert them to physical
units using the sensor's configured range and scale in your application.
Alternatively, call `init_with_delay(&mut delay)` with an
`embedded_hal_async::delay::DelayNs` implementation.

## Hardware notes

For the GY-BMI160 (HW-661) module in I2C mode:

- Connect `CS` to 3.3 V.
- Connect `SA0` to GND for address `0x68`, or to 3.3 V for `0x69`.
- Connect `SCL`, `SDA`, power, and ground to the host; add pull-ups if required
  by the bus.

Pass the configured address to `Bmi160::new`; use `set_address` to change it.

## Ecosystem integration

`signals::ACCEL_SIGNAL` and `signals::GYRO_SIGNAL` use the selected backend's
synchronization primitives. Publish a sample with `signal(sample)` and consume
it with `wait().await`. Each signal holds only the latest value and supports a
single consumer; a new sample replaces an unread one. Applications may instead
use their own channels or signals.

See [CHANGELOG.md](CHANGELOG.md) for release history.

## License

Licensed under GPL-2.0-or-later.

Copyright (C) 2026 Jorge Andre Castro