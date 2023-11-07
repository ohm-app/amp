pub mod power {
    #[cfg_attr(feature = "strum_derive", derive(strum_macros::Display))]
    #[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum BatteryState {
        Charging(u8),
        Discharging(u8),
        Percent(u8),
        Unknown,
    }

    #[cfg_attr(feature = "strum_derive", derive(strum_macros::Display))]
    #[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum BatteryType {
        LeftRightCase(BatteryState, BatteryState, BatteryState),
        Single(BatteryState),
    }

    #[cfg_attr(feature = "strum_derive", derive(strum_macros::Display))]
    #[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum PowerState {
        On,
        Off,
    }
}

#[cfg_attr(feature = "strum_derive", derive(strum_macros::Display))]
#[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DeviceType {
    Headphones,
    Earbuds,
}

#[cfg_attr(feature = "strum_derive", derive(strum_macros::Display))]
#[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AmbientSound {
    NoiseCancelling(u8),
    Off,
    AmbientSound(u8),
}
