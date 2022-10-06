use crate::definitions::*;
use crate::led_animation::{ OFF, SOLID };
use crate::ui_logic::BatteryStatusCfg;

use sodium_rust::{ Cell, SodiumCtx, Stream };

use std::time::{ Duration, Instant };

#[derive(Clone)]
pub struct BatteryStatusCtx {
    pub displaying: Cell<bool>,
    pub updates: Stream<Vec<UiUpdates>>,
}

pub fn battery_status_setup( s: &SodiumCtx, battery_level: &Stream<u8>, gestures: &Stream<Gesture>, time: &Stream<Instant>, cfg: &Cell<BatteryStatusCfg> ) -> BatteryStatusCtx {
    let battery_status_displayed_loop = s.new_cell_loop::<bool>();
    let alarm_loop = s.new_cell_loop::<Option<(Instant, Duration)>>();

    let battery_status_displayed = battery_status_displayed_loop.cell();
    let alarm = alarm_loop.cell();

    let state_of_charge = battery_level.hold( 100 );

    let timer_expired = time
        .snapshot( &alarm, | t_: &Instant, a_: &Option<(Instant, Duration)> | -> (Instant, Option<(Instant, Duration)>) { ( t_.clone(), a_.clone() ) } )
        .filter( | t: &( Instant, Option<(Instant, Duration)> ) | -> bool { match t.1 { Some( alarm_ ) => (t.0 >= alarm_.0 + alarm_.1), _ => false } } )
        .map( | _dummy: &(Instant, Option<(Instant, Duration)>) | () );

    let start_display = gestures
        .snapshot( &battery_status_displayed, | g: &Gesture, visible: &bool | -> ( Gesture, bool ) { ( g.clone(), visible.clone() ) } )
        .filter( | t: &( Gesture, bool ) | -> bool { match t.0 { Gesture::ShowBatteryStatus { display_time } => !t.1, _ => false, } } )
        .map( | t: &(Gesture, bool) | -> Gesture { t.0.clone() } );

    alarm_loop.loop_(&start_display
        .map( | g: &Gesture | -> Option<(Instant, Duration)> { match g { Gesture::ShowBatteryStatus{ display_time } => Some( (Instant::now(), display_time.clone() ) ), _ => None } } )
        .or_else(
            &timer_expired.map( | _: &() | -> Option<(Instant, Duration)> { None } )
        ).hold( None ) );

    battery_status_displayed_loop.loop_( &start_display
        .map( | _: &Gesture | -> bool { true } )
        .or_else(
            &timer_expired.map( | _: &() | -> bool { false } )
        )
        .hold( false ) );

    let updates = start_display
        .snapshot3( &state_of_charge, &cfg, | g: &Gesture, soc: &u8, cfg_: &BatteryStatusCfg | -> (u8, BatteryStatusCfg) { ( soc.clone(), cfg_.clone() ) } )
        .map( | t: &(u8, BatteryStatusCfg) | -> Vec<UiUpdates> { map_battery_status_changes( &t.0, &t.1 ) } )
        .or_else( &battery_level
            .snapshot3( &battery_status_displayed, &cfg, | soc: &u8, displaying: &bool, cfg_: &BatteryStatusCfg| -> (bool, u8, BatteryStatusCfg) { (displaying.clone(), soc.clone(), cfg_.clone()) } )
            .filter( | t: &(bool, u8, BatteryStatusCfg) | -> bool { t.0 } )
            .map( | t: &(bool, u8, BatteryStatusCfg) | -> Vec<UiUpdates> { map_battery_status_changes( &t.1, &t.2 ) } )
        )
        .or_else( &timer_expired
            .map( | _: &() | -> Vec<UiUpdates> {
                    let mut v: Vec<UiUpdates> = Vec::new();
                    v.push( UiUpdates::LedAnimation{ led: LedName::Green3, animation: OFF } );
                    v.push( UiUpdates::LedAnimation{ led: LedName::Red3, animation: OFF } );
                    v
                }
            )
        );

    BatteryStatusCtx{
        displaying: battery_status_displayed,
        updates,

    }
}

fn map_battery_status_changes( soc: &u8, cfg: &BatteryStatusCfg ) -> Vec<UiUpdates> {
    let mut v: Vec<UiUpdates> = Vec::new();
    if soc > &cfg.critical && soc <= &cfg.low {
        v.push( UiUpdates::LedAnimation{ led: LedName::Green3, animation: SOLID } );
        v.push( UiUpdates::LedAnimation{ led: LedName::Red3, animation: SOLID } );
    } else if soc <= &cfg.critical {
        v.push( UiUpdates::LedAnimation{ led: LedName::Green3, animation: OFF } );
        v.push( UiUpdates::LedAnimation{ led: LedName::Red3, animation: SOLID } );
    } else {
        v.push( UiUpdates::LedAnimation{ led: LedName::Green3, animation: SOLID } );
        v.push( UiUpdates::LedAnimation{ led: LedName::Red3, animation: OFF } );
    }
    v
}