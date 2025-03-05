use crate::types::{AccRange, AxisData, GyrRangeVal, Odr};

const GRAVITY: f32 = 9.80665;

// #[derive(Debug, Default, Clone, Copy)]
// #[repr(i16)]
// pub enum GRange {
//     #[default]
//     Two = 2,
//     Four = 4,
//     Eight = 8,
//     Sixteen = 16,
// }

/// Raw value to meters per second squared.
fn raw_to_mps2(value: i16, acc_range: AccRange) -> f32 {
    GRAVITY * (acc_range.to_i16() as f32) * (value as f32 / 0x8000i32 as f32)
}

// #[derive(Debug, Default, Clone, Copy)]
// #[repr(i16)]
// pub enum Dps {
//     Dps125 = 125,
//     Dps250 = 250,
//     Dps500 = 500,
//     Dps1000 = 1000,
//     #[default]
//     Dps2000 = 2000,
// }

/// Raw value to degrees per second.
fn raw_to_dps(value: i16, gyr_range: GyrRangeVal, odr: Odr) -> f32 {
    odr.to_hz() * (gyr_range.to_i16() as f32) * (value as f32 / (i32::MAX as f32))
}

#[derive(Debug, Clone, Copy)]
pub struct GyroData {
    /// X axis data.
    pub x: f32,
    /// Y axis data.
    pub y: f32,
    /// Z axis data.
    pub z: f32,
}

impl GyroData {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AccData {
    /// X axis data.
    pub x: f32,
    /// Y axis data.
    pub y: f32,
    /// Z axis data.
    pub z: f32,
}

impl AccData {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl AxisData {
    pub fn raw_to_gyr(&self, gyr_range: GyrRangeVal, odr: Odr) -> GyroData {
        GyroData::new(
            raw_to_dps(self.x, gyr_range, odr),
            raw_to_dps(self.y, gyr_range, odr),
            raw_to_dps(self.z, gyr_range, odr),
        )
    }

    pub fn raw_to_mps2(&self, acc_range: AccRange) -> AccData {
        AccData::new(
            raw_to_mps2(self.x, acc_range),
            raw_to_mps2(self.y, acc_range),
            raw_to_mps2(self.z, acc_range),
        )
    }
}
