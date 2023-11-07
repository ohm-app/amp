use crate::error::AmpResult;

use super::{power::BatteryType, AmbientSound};

pub(crate) trait Device {
    fn battery_level(&self) -> AmpResult<BatteryType>;
    fn name(&self) -> &'static str;
}

pub trait AmbientSoundControl {
    fn set_soundcontrol(&self, level: AmbientSound) -> AmpResult<AmbientSound>;
}
