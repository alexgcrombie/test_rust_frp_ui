use crate::definitions::*;
use crate::led_animation::*;

use sodium_rust::{ Cell, SodiumCtx, Stream };

pub struct ChargerCtx {
    pub updates: Stream<Vec<UiUpdates>>,
    pub state: Cell<ChargingStatus>,
}

pub fn charger_setup( _s: &SodiumCtx, charger_events: &Stream<ChargingStatus> ) -> ChargerCtx {
    let state = charger_events.map( | e: &ChargingStatus | -> ChargingStatus { e.clone() } ).hold( ChargingStatus::Disconnected );

    let updates1 = charger_events.map( | e: &ChargingStatus | -> Vec<UiUpdates> {
        let mut v: Vec<UiUpdates> = Vec::new();
        match e {
            ChargingStatus::Disconnected => v.push( UiUpdates::LedAnimation{ led: LedName::Green1, animation: OFF } ),
            ChargingStatus::Charging => v.push( UiUpdates::LedAnimation{ led: LedName::Green1, animation: THROB } ),
            ChargingStatus::Charged => v.push( UiUpdates::LedAnimation{ led: LedName::Green1, animation: SOLID } ),
        }
        v
    } );

    ChargerCtx {
        state,
        updates: updates1,
    }
}