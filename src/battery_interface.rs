use crate::battery_info::BatteryInfo;

pub mod upower;

pub trait BatteryInterface {
    fn battery_info() -> std::result::Result<BatteryInfo, impl Into<Box<dyn std::error::Error + 'static>>>;
}
