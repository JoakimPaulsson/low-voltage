pub mod battery_level;
use battery_level::*;

pub mod device_state;
use device_state::*;

pub mod device_type;
use device_type::*;

pub mod warning_level;
use warning_level::*;

pub mod time_until;
use time_until::*;

use crate::battery_interface::BatteryInterface;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentage(f64);

impl std::ops::Deref for Percentage {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<f64> for Percentage {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerSupply(bool);

impl std::ops::Deref for PowerSupply {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<bool> for PowerSupply {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconName(String);

impl std::ops::Deref for IconName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for IconName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct BatteryInfo {
    pub(crate) device_type: Option<DeviceType>,
    pub(crate) device_state: Option<DeviceState>,
    pub(crate) percentage: Option<Percentage>,
    pub(crate) power_supply: Option<PowerSupply>,
    pub(crate) battery_level: Option<BatteryLevel>,
    pub(crate) icon_name: Option<IconName>,
    pub(crate) time_until: Option<TimeUntil>,
    pub(crate) warning_level: Option<WarningLevel>,
}

pub enum BatteryInfoProperties {
    DeviceType(DeviceType),
    DeviceState(DeviceState),
    Percentage(Percentage),
    PowerSupply(PowerSupply),
    BatteryLevel(BatteryLevel),
    IconName(IconName),
    TimeUntil(TimeUntil),
    WarningLevel(WarningLevel),
}

impl BatteryInfoProperties {
    pub fn insert_property_by_mut_ref(self, batt_info: &mut BatteryInfo) {
        match self {
            Self::DeviceType(device_type) => {
                batt_info.device_type = Some(device_type);
            }
            Self::DeviceState(device_state) => batt_info.device_state = Some(device_state),
            Self::Percentage(percentage) => batt_info.percentage = Some(percentage),
            Self::PowerSupply(power_supply) => batt_info.power_supply = Some(power_supply),
            Self::BatteryLevel(battery_level) => batt_info.battery_level = Some(battery_level),
            Self::IconName(icon_name) => batt_info.icon_name = Some(icon_name),
            Self::TimeUntil(time_until) => batt_info.time_until = Some(time_until),
            Self::WarningLevel(warning_level) => batt_info.warning_level = Some(warning_level),
        };
    }
    pub fn insert_property(self, mut batt_info: BatteryInfo) {
        match self {
            Self::DeviceType(device_type) => {
                batt_info.device_type = Some(device_type);
            }
            Self::DeviceState(device_state) => batt_info.device_state = Some(device_state),
            Self::Percentage(percentage) => batt_info.percentage = Some(percentage),
            Self::PowerSupply(power_supply) => batt_info.power_supply = Some(power_supply),
            Self::BatteryLevel(battery_level) => batt_info.battery_level = Some(battery_level),
            Self::IconName(icon_name) => batt_info.icon_name = Some(icon_name),
            Self::TimeUntil(time_until) => batt_info.time_until = Some(time_until),
            Self::WarningLevel(warning_level) => batt_info.warning_level = Some(warning_level),
        }
    }
}

impl Default for BatteryInfo {
    fn default() -> Self {
        Self {
            device_type: None,
            device_state: None,
            percentage: None,
            power_supply: None,
            battery_level: None,
            icon_name: None,
            time_until: None,
            warning_level: None,
        }
    }
}

impl BatteryInfo {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_propertry(&mut self, prop: BatteryInfoProperties) {
        prop.insert_property_by_mut_ref(self);
    }

    pub fn get<T: BatteryInterface>() -> Option<Self> {
        <T as BatteryInterface>::battery_info().ok()
    }
}

#[cfg(test)]
mod tests {
    use crate::battery_interface::upower::UPower;

    use super::*;

    #[test]
    fn canoncial_use_case() {

        let batt_info = BatteryInfo::get::<UPower>();

        insta::assert_debug_snapshot!(batt_info.is_some(), @"true");

        let batt_info = batt_info.unwrap();

        let device_type = batt_info.device_type;
        let device_state = batt_info. device_state;
        let percentage = batt_info. percentage;
        let power_supply = batt_info. power_supply;
        let battery_level = batt_info. battery_level;
        let icon_name = batt_info. icon_name;
        let time_until = batt_info. time_until;
        let warning_level = batt_info. warning_level;

        
        insta::assert_debug_snapshot!(device_type.is_some(), @"true");
        insta::assert_debug_snapshot!(device_state.is_some(), @"true");
        insta::assert_debug_snapshot!(percentage.is_some(), @"true");
        insta::assert_debug_snapshot!(power_supply.is_some(), @"true");
        insta::assert_debug_snapshot!(battery_level.is_some(), @"true");
        insta::assert_debug_snapshot!(icon_name.is_some(), @"true");
        insta::assert_debug_snapshot!(time_until.is_some(), @"true");
        insta::assert_debug_snapshot!(warning_level.is_some(), @"true");
    }
}
