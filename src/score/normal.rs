use std::f32::consts::SQRT_2;

mod c {
    extern "C" {
        pub fn erff(x: f32) -> f32;
    }
}

#[inline]
pub fn erff(x: f32) -> f32 {
    unsafe { c::erff(x) }
}

pub struct Normal {
    mean: f32,
    std_dev: f32,
}

impl Normal {
    pub fn new(mean: f32, std_dev: f32) -> Self {
        Normal { mean, std_dev }
    }

    pub fn cdf(&self, x: f32) -> f32 {
        0.5 * (1.0 + erff((x - self.mean) / (self.std_dev * SQRT_2)))
    }
}
