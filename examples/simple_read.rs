//! Minimal example of reading raw BMI160 samples.

use embedded_hal_async::i2c::I2c;
use hazard3_gy_bmi160::{Bmi160, BmiData};

/// Initializes the sensor and reads one accelerometer and gyroscope sample.
pub async fn read_samples<I>(i2c: I) -> Result<(BmiData, BmiData), I::Error>
where
    I: I2c,
{
    let mut imu = Bmi160::new(i2c, 0x68);
    imu.init().await?;

    let accelerometer = imu.read_accel().await?;
    let gyroscope = imu.read_gyro().await?;

    Ok((accelerometer, gyroscope))
}
