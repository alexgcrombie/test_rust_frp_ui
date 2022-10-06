use crate::led_animation;
use crate::led_animation::LedAnimation;

#[derive(Clone,Debug)]
pub struct Led {
    pub muted: bool,
    pub brightness: u8,
    pub animation: LedAnimation
}

#[derive(Clone,Debug)]
pub struct UiLayout {
    pub red_1: Led,
    pub green_1: Led,
    pub blue_1: Led,

    pub red_2: Led,
    pub green_2: Led,
    pub blue_2: Led,

    pub red_3: Led,
    pub green_3: Led,
    pub blue_3: Led,
}

impl Led {
    pub fn default() -> Led {
        Led{
            muted: false,
            brightness: 64,
            animation: led_animation::OFF,
        }
    }
}

impl UiLayout {
    pub fn default() -> UiLayout {
        UiLayout {
            red_1: Led::default(),
            green_1: Led::default(),
            blue_1: Led::default(),
            red_2: Led::default(),
            green_2: Led::default(),
            blue_2: Led::default(),
            red_3: Led::default(),
            green_3: Led::default(),
            blue_3: Led::default(),
        }
    }

    pub fn print( &self ) {
        println!( "{:?}", self );
    }
}