// Copyright (C) 2026 Jorge Andre Castro
// GPL-2.0-or-later

#![no_std]
#![forbid(unsafe_code)]

//! # hazard3-gy-bmi160
//!
//! Asynchronous `no_std` I2C driver for the Bosch BMI160 IMU.
//!
//! The driver provides raw accelerometer and gyroscope data. Select the
//! `hazard3` backend (default) for RP2350 RISC-V or the `embassy` backend for
//! ARM/Cortex-M. Exactly one backend must be enabled.

#[cfg(all(feature = "hazard3", feature = "embassy"))]
compile_error!("Features `hazard3` and `embassy` are mutually exclusive.");

#[cfg(not(any(feature = "hazard3", feature = "embassy")))]
compile_error!("Enable exactly one backend: `hazard3` or `embassy`.");

#[cfg(feature = "hazard3")]
use hazard3_time_backend::Delay;
#[cfg(all(feature = "embassy", not(feature = "hazard3")))]
use embassy_time::Delay;

pub mod signals;

use core::marker::PhantomData;
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;

/// Raw three-axis motion data (X, Y, Z), represented as signed 16-bit integers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BmiData {
    /// Raw value for the X axis.
    pub x: i16,
    /// Raw value for the Y axis.
    pub y: i16,
    /// Raw value for the Z axis.
    pub z: i16,
}

/// BMI160 driver instance.
///
/// Works with any device implementing `embedded_hal_async::i2c::I2c`.
pub struct Bmi160<'d, I>
where
    I: I2c,
{
    i2c: I,
    /// Configured I2C address (0x68 or 0x69).
    pub addr: u8,
    _phantom: PhantomData<&'d ()>,
}

impl<'d, I> Bmi160<'d, I>
where
    I: I2c,
{
    /// Creates a new driver instance.
    ///
    /// * `i2c` - I2C bus or shared I2C device.
    /// * `addr` - Device address, usually `0x68`.
    pub fn new(i2c: I, addr: u8) -> Self {
        Self {
            i2c,
            addr,
            _phantom: PhantomData,
        }
    }

    /// Updates the I2C address.
    pub fn set_address(&mut self, new_addr: u8) {
        self.addr = new_addr;
    }

    /// Puts the accelerometer and gyroscope into normal mode.
    ///
    /// The application must provide a time driver for the selected backend.
    #[cfg(any(feature = "hazard3", feature = "embassy"))]
    pub async fn init(&mut self) -> Result<(), I::Error> {
        self.init_with_delay(&mut Delay).await
    }

    /// Puts the accelerometer and gyroscope into normal mode using a
    /// caller-provided async delay.
    pub async fn init_with_delay<D>(&mut self, delay: &mut D) -> Result<(), I::Error>
    where
        D: DelayNs,
    {
        // Put the accelerometer into normal mode.
        self.i2c.write(self.addr, &[0x7E, 0x11]).await?;
        delay.delay_ms(10).await;

        // Put the gyroscope into normal mode.
        self.i2c.write(self.addr, &[0x7E, 0x15]).await?;
        delay.delay_ms(10).await;

        Ok(())
    }

    /// Reads raw gyroscope data from registers `0x0C` through `0x11`.
    pub async fn read_gyro(&mut self) -> Result<BmiData, I::Error> {
        let mut data = [0u8; 6];
        self.i2c.write_read(self.addr, &[0x0C], &mut data).await?;
        Ok(BmiData {
            x: i16::from_le_bytes([data[0], data[1]]),
            y: i16::from_le_bytes([data[2], data[3]]),
            z: i16::from_le_bytes([data[4], data[5]]),
        })
    }

    /// Reads raw accelerometer data from registers `0x12` through `0x17`.
    pub async fn read_accel(&mut self) -> Result<BmiData, I::Error> {
        let mut data = [0u8; 6];
        self.i2c.write_read(self.addr, &[0x12], &mut data).await?;
        Ok(BmiData {
            x: i16::from_le_bytes([data[0], data[1]]),
            y: i16::from_le_bytes([data[2], data[3]]),
            z: i16::from_le_bytes([data[4], data[5]]),
        })
    }
}