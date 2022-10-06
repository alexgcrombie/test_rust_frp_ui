mod battery_status;
mod charger;

use crate::definitions::*;
use crate::inputs::Inputs;
use crate::ui_layout::Led;
use crate::ui_layout::UiLayout;

use crate::ui_logic::battery_status::battery_status_setup;
use crate::ui_logic::charger::charger_setup;

use sodium_rust::{ Cell, SodiumCtx, Stream };
use crate::led_animation::LedAnimation;

pub struct Outputs {
    pub ui_state: Cell<UiLayout>,
    pub battery_status_showing: Cell<bool>,
    pub updates: Stream<Vec<UiUpdates>>,
}

#[derive(Copy, Clone)]
pub struct BatteryStatusCfg {
    pub low: u8,
    pub critical: u8
}

pub fn setup_ui( s: &SodiumCtx, inputs: &Inputs ) -> Outputs {
    let battery_status_cfg = s.new_cell(BatteryStatusCfg{ low: 15, critical: 3 } );

    let ui_state_loop = s.new_cell_loop::<UiLayout>();

    let ui_state = ui_state_loop.cell();

    let charger = charger_setup( s, &inputs.streams.charging_status );
    let battery_status = battery_status_setup( s, &inputs.streams.battery_level, &inputs.streams.gestures, &inputs.streams.time, &battery_status_cfg );

    let updates = charger.updates.or_else( &battery_status.updates );

    let ui_state_updates = updates.snapshot( &ui_state, |updates: &Vec<UiUpdates>, state: &UiLayout| -> UiLayout {
        let mut ui_state_ = state.clone();
        for update in updates {
            match update {
                UiUpdates::LedAnimation{ led, animation } => { ui_state_ = update_led_animation( &ui_state_, &led, &animation ); },
            }
        }
        ui_state_
    } );

    ui_state_loop.loop_( &ui_state_updates.hold( UiLayout::default() ) );

    Outputs {
        ui_state,
        battery_status_showing: battery_status.displaying,
        updates,
    }
}

fn update_led_animation(ui_state: &UiLayout, led: &LedName, animation: &LedAnimation ) -> UiLayout {
    match led {
        LedName::Red1 => UiLayout { red_1: Led{ animation: animation.clone(), ..ui_state.red_1.clone() }, ..ui_state.clone() },
        LedName::Green1 => UiLayout { green_1: Led{ animation: animation.clone(), ..ui_state.green_1.clone() }, ..ui_state.clone() },
        LedName::Blue1 => UiLayout { blue_1: Led{ animation: animation.clone(), ..ui_state.blue_1.clone() }, ..ui_state.clone() },

        LedName::Red2 => UiLayout { red_2: Led{ animation: animation.clone(), ..ui_state.red_2.clone() }, ..ui_state.clone() },
        LedName::Green2 => UiLayout { green_2: Led{ animation: animation.clone(), ..ui_state.green_2.clone() }, ..ui_state.clone() },
        LedName::Blue2 => UiLayout { blue_2: Led{ animation: animation.clone(), ..ui_state.blue_2.clone() }, ..ui_state.clone() },

        LedName::Red3 => UiLayout { red_3: Led{ animation: animation.clone(), ..ui_state.red_3.clone() }, ..ui_state.clone() },
        LedName::Green3 => UiLayout { green_3: Led{ animation: animation.clone(), ..ui_state.green_3.clone() }, ..ui_state.clone() },
        LedName::Blue3 => UiLayout { blue_3: Led{ animation: animation.clone(), ..ui_state.blue_3.clone() }, ..ui_state.clone() },
    }
}