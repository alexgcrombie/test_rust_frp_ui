pub enum UiEvents {
    ChargerNotPresent,
    ChargerPresent{ charged: bool },

    BatteryLevel{ level: i32 }
}