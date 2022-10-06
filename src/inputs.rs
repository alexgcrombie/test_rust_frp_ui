use crate::definitions::*;

use sodium_rust::Stream;

use std::time::Instant;

pub struct InputStreams {
    pub charging_status: Stream<ChargingStatus>,
    pub battery_level: Stream<u8>,

    pub gestures: Stream<Gesture>,

    pub time: Stream<Instant>,
}

pub struct Inputs {
    pub streams: InputStreams
}