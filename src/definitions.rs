use crate::led_animation::LedAnimation;

use std::time::Duration;

#[derive(Copy, Clone, PartialEq)]
pub enum ChargingStatus {
    Disconnected,
    Charging,
    Charged,
}

#[derive(Copy, Clone, Debug)]
pub enum LedName {
    Red1,
    Green1,
    Blue1,

    Red2,
    Green2,
    Blue2,

    Red3,
    Green3,
    Blue3,

}

#[derive(Copy, Clone)]
pub enum UiUpdates {
    LedAnimation{ led: LedName, animation: LedAnimation },
    //TODO: LedBrightness{ led: LedName, brightness: u8 },
}

#[derive(Copy, Clone)]
pub enum Gesture {
    ShowBatteryStatus{ display_time: Duration },
}