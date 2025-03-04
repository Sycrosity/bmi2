use crate::types::AxisData;

const GRAVITY: f32 = 9.80665;

#[derive(Debug, Default, Clone, Copy)]
#[repr(i16)]
pub enum GRange {
    #[default]
    Two = 2,
    Four = 4,
    Eight = 8,
    Sixteen = 16,
}

fn raw_to_mps2(value: i16, g_range: GRange) -> f32 {
    GRAVITY * (g_range as i32 as f32) * (value as f32 / i16::MAX as f32)
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(i16)]
pub enum Dps {
    Dps125 = 125,
    Dps250 = 250,
    Dps500 = 500,
    Dps1000 = 1000,
    #[default]
    Dps2000 = 2000,
}

fn raw_to_dps(value: i16, dps: Dps) -> f32 {
    (dps as i16 as f32) * (value as f32 / (i32::MAX as f32))
}

#[derive(Debug, Clone, Copy)]
pub struct GyroData {
x: f32,
y: f32,
z: f32,
}

impl GyroData {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AccData {
    x: f32,
    y: f32,
    z: f32,
}

impl AccData {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl AxisData {
    pub fn raw_to_dps(&mut self, dps: Dps) -> GyroData {
        GyroData::new(
            raw_to_dps(self.x, dps),
            raw_to_dps(self.y, dps),
            raw_to_dps(self.z, dps),
        )
    }

    pub fn raw_to_mps2(&mut self, g_range: GRange) -> AccData {
        AccData::new(
            raw_to_mps2(self.x, g_range),
            raw_to_mps2(self.y, g_range),
            raw_to_mps2(self.z, g_range),
        )
    }
}
