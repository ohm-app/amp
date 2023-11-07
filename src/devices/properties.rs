pub mod power {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum_macros::Display)]
    pub enum BatteryState {
        Charging(u8),
        Discharging(u8),
        Percent(u8),
        Unknown,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum_macros::Display)]
    pub enum BatteryType {
        LeftRightCase(BatteryState, BatteryState, BatteryState),
        Single(BatteryState),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum_macros::Display)]
    pub enum PowerState {
        On,
        Off,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum_macros::Display)]
pub enum DeviceType {
    Headphones,
    Earbuds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum_macros::Display)]
pub enum AmbientSound {
    NoiseCancelling(u8),
    Off,
    AmbientSound(u8),
}
