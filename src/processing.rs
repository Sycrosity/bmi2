use crate::types::AxisData;

const GRAVITY: f32 = 9.80665;

#[derive(Debug, Default)]
#[repr(i16)]
enum GRange {
    #[default]
    Two = 2,
    Four = 4,
    Eight = 8,
    Sixteen = 16,
}

fn raw_to_mps2(value: i16, g_range: GRange) -> f32 {
   
    GRAVITY * (g_range as i32 as f32) * (value as f32 / i16::MAX as f32)

}

#[derive(Debug, Default)]
#[repr(i16)]
enum Dps {

    Dps125 = 125,
    Dps250 = 250,
    Dps500 = 500,
    Dps1000 = 1000,
    #[default]
    Dps2000 = 2000,

}

fn raw_to_dps(value: i16, dps: Dps) -> f32 {

    (dps as i16 as f32) * ( value as f32 / (i32::MAX as f32))

}

impl AxisData {
    pub fn raw_to_dps(&mut self, dps: Dps) {

        self.x = raw_to_dps(self.x, dps);
        self.y = raw_to_dps(self.y, dps);
        self.z = raw_to_dps(self.z, dps);

    }

    pub fn raw_to_mps2(&mut self, g_range: GRange) {

        self.x = raw_to_mps2(self.x, g_range);
        self.y = raw_to_mps2(self.y, g_range);
        self.z = raw_to_mps2(self.z, g_range);

    }
}