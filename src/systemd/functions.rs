use std::ops::Deref;

use crate::dbus_interface::DBusInterface;

use super::{dbus::manager::OrgFreedesktopSystemd1Manager, dto::{UnitDto, ServiceDto, UnitListEntry}};

pub fn load_unit_data(dbus: &DBusInterface, unit_name: &str) -> Result<UnitDto, dbus::Error> {
    let manager = dbus.systemd_manager();
    let unit_path = manager.load_unit(unit_name)?;
    let unit_proxy = dbus.systemd_unit(unit_path.deref());
    let mut unit = UnitDto::create_from_proxy(&unit_proxy)?;

    if unit_name.ends_with(".service") {
        let service_proxy = dbus.systemd_service(unit_path.deref());
        unit.add_service(ServiceDto::create_from_proxy(&service_proxy)?);
    }

    Ok(unit)
}

pub fn list_units(dbus: &DBusInterface) -> Result<Vec<UnitListEntry>, dbus::Error> {
    let manager = dbus.systemd_manager();
    let unit_paths = manager.list_units()?;
    return Ok(unit_paths.into_iter().map(UnitListEntry::from).collect());
}