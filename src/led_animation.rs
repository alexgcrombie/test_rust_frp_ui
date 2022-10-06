#[derive(Copy, Clone,Debug)]
pub struct LedAnimation {
    minimum_amplitude: u8,
    maximum_amplitude: u8,
    bottom_time: u16,
    ramp_up_time: u16,
    top_time: u16,
    ramp_down_time: u16,
}

pub const OFF: LedAnimation = LedAnimation{ minimum_amplitude: 0, maximum_amplitude: 0, bottom_time: 0, ramp_up_time: 0, top_time: 0, ramp_down_time: 0 };
pub const SOLID: LedAnimation = LedAnimation{ minimum_amplitude: 255, maximum_amplitude: 255, bottom_time: 0, ramp_up_time: 0, top_time: 0, ramp_down_time: 0 };
pub const THROB: LedAnimation = LedAnimation{ minimum_amplitude: 1, maximum_amplitude: 255, bottom_time: 0, ramp_up_time: 750, top_time: 0, ramp_down_time: 750 };