#![cfg_attr(not(test), no_std)]

use interface::imu::Accelerometer;


pub mod interface;
pub mod peripheral;
pub mod state;
pub mod log;
pub mod ahrs;
pub mod math;

pub struct FlightController<L: log::Logger, A: Accelerometer<f32>> {
    log: L,
    accel: A,
}
