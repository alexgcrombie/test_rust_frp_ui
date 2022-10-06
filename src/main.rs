mod definitions;
mod inputs;
mod led_animation;
mod test_util;
mod ui_layout;
mod ui_logic;

use crate::definitions::{ChargingStatus, Gesture};
use crate::test_util::TestInputSinks;
use crate::ui_logic::{ setup_ui };

use sodium_rust::SodiumCtx;

use std::thread;
use std::time::{ Duration, Instant };

fn main() {
    let ctx = SodiumCtx::new();

    let t1 = Instant::now();

    let input_sinks = TestInputSinks::new( &ctx );
    let inputs = input_sinks.create_inputs();

    let outputs = setup_ui( &ctx, &inputs );

    let t2 = Instant::now();

    // Test cases start
    input_sinks.stream_sinks.charger_status.send( ChargingStatus::Charging );

    let t3 = Instant::now();

    println!("Charging:");
    outputs.ui_state.sample().print();

    let t4 = Instant::now();

    input_sinks.stream_sinks.charger_status.send( ChargingStatus::Charged );

    let t5 = Instant::now();

    println!("Charged:");
    outputs.ui_state.sample().print();

    let t6 = Instant::now();

    input_sinks.stream_sinks.charger_status.send( ChargingStatus::Disconnected );

    let t7 = Instant::now();

    println!("Disconnected:");
    outputs.ui_state.sample().print();

    let t8 = Instant::now();

    input_sinks.stream_sinks.time.send( Instant::now() );

    let t9 = Instant::now();

    input_sinks.stream_sinks.gestures.send( Gesture::ShowBatteryStatus{ display_time: Duration::from_secs( 5 ) } );

    let t10 = Instant::now();

    println!("Show battery:");
    println!( "Showing: {}", outputs.battery_status_showing.sample() );
    outputs.ui_state.sample().print();

    let t11 = Instant::now();

    thread::sleep( Duration::from_millis( 5100 ) );

    let t12 = Instant::now();

    input_sinks.stream_sinks.time.send( t12.clone() );

    let t13 = Instant::now();

    println!("Battery Status Timeout:");
    println!( "Showing: {}", outputs.battery_status_showing.sample() );
    outputs.ui_state.sample().print();

    let t14 = Instant::now();

    println!( "Timings:" );
    println!( "  setup: {}ms", (t2 - t1).as_millis() );
    println!( "  send( charging ): {}ms", (t3 - t2).as_millis() );
    println!( "  print(): {}us", (t4 - t3).as_micros() );
    println!( "  send( charged ): {}ms", (t5 -t4).as_millis() );
    println!( "  print(): {}us", (t6 - t5).as_micros() );
    println!( "  send( disconnected ): {}ms", (t7 - t6).as_millis() );
    println!( "  print(): {}us", (t8 - t7).as_micros() );
    println!( "  send( time ): {}ms", (t9 - t8).as_millis() );
    println!( "  send( show_battery ): {}ms", (t10 - t9).as_millis() );
    println!( "  print(): {}us", (t11 - t10).as_micros() );
    println!( "  send( time ): {}ms", (t13 - t12).as_millis() );
    println!( "  print(): {}us", (t14 - t13).as_micros() );
}
