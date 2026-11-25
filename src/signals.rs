// Copyright (C) 2026 Jorge Andre Castro
// GPL-2.0-or-later

//! Single-consumer signals for sharing the latest IMU samples between tasks.
//!
//! The application must provide a `critical-section` implementation.

use crate::BmiData;
#[cfg(all(feature = "embassy", not(feature = "hazard3")))]
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
#[cfg(feature = "hazard3")]
use hazard3_sync_backend::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};

/// Latest accelerometer sample.
#[cfg(any(feature = "hazard3", feature = "embassy"))]
pub static ACCEL_SIGNAL: Signal<CriticalSectionRawMutex, BmiData> = Signal::new();

/// Latest gyroscope sample.
#[cfg(any(feature = "hazard3", feature = "embassy"))]
pub static GYRO_SIGNAL: Signal<CriticalSectionRawMutex, BmiData> = Signal::new();
