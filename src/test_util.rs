use crate::definitions::*;
use crate::inputs::{Inputs, InputStreams};

use sodium_rust::{ SodiumCtx, StreamSink };

use std::time::Instant;

pub struct TestInputStreamSinks {
    pub charger_status: StreamSink<ChargingStatus>,
    pub battery_level: StreamSink<u8>,

    pub gestures: StreamSink<Gesture>,

    pub time: StreamSink<Instant>,
}

pub struct TestInputSinks {
    pub stream_sinks: TestInputStreamSinks,
}

impl TestInputSinks {
    pub fn new( s: &SodiumCtx ) -> TestInputSinks {
        TestInputSinks {
            stream_sinks: TestInputStreamSinks{
                charger_status: s.new_stream_sink(),
                battery_level: s.new_stream_sink(),

                gestures: s.new_stream_sink(),

                time: s.new_stream_sink(),
            },
        }
    }

    pub fn create_inputs( &self ) -> Inputs {
        Inputs {
            streams: InputStreams {
                charging_status: self.stream_sinks.charger_status.stream(),
                battery_level: self.stream_sinks.battery_level.stream(),

                gestures: self.stream_sinks.gestures.stream(),

                time: self.stream_sinks.time.stream(),
            }
        }
    }
}