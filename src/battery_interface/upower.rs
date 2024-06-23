use std::{
    collections::HashMap,
    ops::Deref,
    thread::sleep,
};

use anyhow::{
    anyhow,
    bail,
};
use once_cell::sync::Lazy;
use zbus::names::InterfaceName;
use zbus::zvariant::Optional;
use zbus::{
    blocking::fdo::PropertiesProxy,
    proxy::CacheProperties,
    zvariant,
};

use crate::battery_info::*;

mod utils;
use utils::*;

use super::*;

type DBusConnection = zbus::blocking::Connection;

#[zbus::proxy(
    interface = "org.freedesktop.UPower",
    default_service = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower",
    gen_async = false
)]
trait UPower {
    fn get_display_device(&self) -> zbus::Result<zvariant::OwnedObjectPath>;
}

pub struct UPower {
    proxy: UPowerProxy<'static>,
    properties_proxy: PropertiesProxy<'static>,
}

impl UPower {
    pub fn new() -> anyhow::Result<&'static Self> {
        let clj = || DBUS_UPOWER.deref().as_ref();
        retry_result_with_delay::<'static, UPower, 100>(clj)
    }

    pub fn get_display_device(&self) -> anyhow::Result<zvariant::OwnedObjectPath> {
        Ok(self.proxy.get_display_device()?)
    }

    pub fn get_all_display_device_properties(
        &self,
    ) -> anyhow::Result<HashMap<String, zvariant::OwnedValue>> {
        Ok(self
            .properties_proxy
            .get_all(Optional::<InterfaceName<'static>>::from(
                InterfaceName::from_static_str("org.freedesktop.UPower.Device").ok(),
            ))?)
    }

    pub fn battery_info(&self) -> anyhow::Result<BatteryInfo> {
        let disp_dev_props = self.get_all_display_device_properties()?;

        let mut batt_info = BatteryInfo::default();

        disp_dev_props.iter().try_for_each(|(k, v)| {
            anyhow::Ok(match k.as_str() {
                "BatteryLevel" => {
                    let prop = BatteryInfoProperties::BatteryLevel(
                        v.downcast_ref::<u32>()
                            .map_err(|e| {
                                anyhow!(
                                    "BatteryLevel: error: {:?}, value: {:?}, key: {:?}",
                                    e,
                                    v,
                                    k
                                )
                            })?
                            .into(),
                    );

                    batt_info.set_propertry(prop);
                }

                "IconName" => {
                    let prop = BatteryInfoProperties::IconName(
                        v.try_to_owned()
                            .map_err(|e| {
                                anyhow!("IconName: error: {:?}, value: {:?}, key: {:?}", e, v, k)
                            })?
                            .to_string()
                            .into(),
                    );

                    batt_info.set_propertry(prop);
                }

                "Type" => {
                    let prop = BatteryInfoProperties::DeviceType(
                        v.downcast_ref::<u32>()
                            .map_err(|e| {
                                anyhow!("Type: error: {:?}, value: {:?}, key: {:?}", e, v, k)
                            })?
                            .into(),
                    );

                    batt_info.set_propertry(prop);
                }

                "TimeToEmpty" | "TimeToFull" => {
                    let value = v.downcast_ref().map_err(|e| {
                        anyhow!(
                            "'TimeToEmpty' | 'TimeToFull': error: {:?}, value: {:?}, key: {:?}",
                            e,
                            v,
                            k
                        )
                    })?;

                    handle_time(k.as_str(), value, &mut batt_info)?;
                }

                "Percentage" => {
                    let prop = BatteryInfoProperties::Percentage(
                        v.downcast_ref::<f64>()
                            .map_err(|e| {
                                anyhow!("Percentage: error: {:?}, value: {:?}, key: {:?}", e, v, k)
                            })?
                            .into(),
                    );

                    batt_info.set_propertry(prop);
                }

                "WarningLevel" => {
                    let prop = BatteryInfoProperties::WarningLevel(
                        v.downcast_ref::<u32>()
                            .map_err(|e| {
                                anyhow!(
                                    "WarningLevel: error: {:?}, value: {:?}, key: {:?}",
                                    e,
                                    v,
                                    k
                                )
                            })?
                            .into(),
                    );

                    batt_info.set_propertry(prop);
                }

                "State" => {
                    let prop = BatteryInfoProperties::DeviceState(
                        v.downcast_ref::<u32>()
                            .map_err(|e| {
                                anyhow!("State: error: {:?}, value: {:?}, key: {:?}", e, v, k)
                            })?
                            .into(),
                    );

                    batt_info.set_propertry(prop);
                }

                "PowerSupply" => {
                    let prop = BatteryInfoProperties::PowerSupply(
                        v.downcast_ref::<bool>()
                            .map_err(|e| {
                                anyhow!("PowerSupply: error: {:?}, value: {:?}, key: {:?}", e, v, k)
                            })?
                            .into()
                    );

                    batt_info.set_propertry(prop);
                }

                &_ => {}
            })
        })?;

        Ok(batt_info)
    }
}

impl BatteryInterface for UPower {
    fn battery_info(
    ) -> std::result::Result<BatteryInfo, impl Into<Box<dyn std::error::Error + 'static>>> {
        let upower = UPower::new()?;
        upower.battery_info()
    }
}


static DBUS_CONNECTION: Lazy<anyhow::Result<DBusConnection>> =
    Lazy::new(|| Ok(DBusConnection::system()?));

static DBUS_UPOWER: Lazy<anyhow::Result<UPower>> = Lazy::new(|| {
    let connection = DBUS_CONNECTION.deref().as_ref();

    let connection = match connection {
        Ok(conn) => Ok(conn),
        Err(_) => retry_result_with_delay::<'static, DBusConnection, 100>(|| {
            DBUS_CONNECTION.deref().as_ref()
        }),
    };

    let connection: &DBusConnection = match connection {
        Ok(c) => c,
        Err(e) => bail!(e),
    };

    let proxy = UPowerProxy::new(connection)?;

    let properties_proxy: PropertiesProxy<'static> = PropertiesProxy::builder(connection)
        .destination("org.freedesktop.UPower")?
        .path("/org/freedesktop/UPower/devices/DisplayDevice")?
        .interface("org.freedesktop.DBus.Properties")?
        .cache_properties(CacheProperties::No)
        .build()?;

    Ok(UPower {
        proxy,
        properties_proxy,
    })
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentage() -> anyhow::Result<()> {
        let upower = UPower::new()?;
        let batt_info = upower.battery_info()?;

        insta::assert_debug_snapshot!(batt_info.percentage.is_some(), @"true");

        Ok(())
    }

    #[test]
    fn battery_info() -> anyhow::Result<()> {
        let upower = UPower::new()?;
        let batt_info = upower.battery_info();

        insta::assert_debug_snapshot!(batt_info.is_ok(), @"true");

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

        Ok(())
    }

    #[test]
    fn display_device() -> anyhow::Result<()> {
        let upower = match UPower::new() {
            Ok(u) => u,
            Err(e) => bail!(e),
        };

        let display_device = upower.get_display_device()?;

        insta::assert_debug_snapshot!(display_device, @r###"
        OwnedObjectPath(
            ObjectPath(
                "/org/freedesktop/UPower/devices/DisplayDevice",
            ),
        )
        "###);

        Ok(())
    }

    #[test]
    fn display_device_properties() -> anyhow::Result<()> {
        let upower = match UPower::new() {
            Ok(u) => u,
            Err(e) => bail!(e),
        };

        let mut disp_dev_props: Vec<String> = upower
            .get_all_display_device_properties()?
            .into_iter()
            .map(|(k, _)| k)
            .collect::<Vec<String>>();

        disp_dev_props.sort();

        insta::assert_debug_snapshot!(disp_dev_props, 
            @r###"
        [
            "BatteryLevel",
            "Capacity",
            "ChargeCycles",
            "Energy",
            "EnergyEmpty",
            "EnergyFull",
            "EnergyFullDesign",
            "EnergyRate",
            "HasHistory",
            "HasStatistics",
            "IconName",
            "IsPresent",
            "IsRechargeable",
            "Luminosity",
            "Model",
            "NativePath",
            "Online",
            "Percentage",
            "PowerSupply",
            "Serial",
            "State",
            "Technology",
            "Temperature",
            "TimeToEmpty",
            "TimeToFull",
            "Type",
            "UpdateTime",
            "Vendor",
            "Voltage",
            "WarningLevel",
        ]
        "###);

        Ok(())
    }
}
