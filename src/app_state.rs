use crate::DBusInterface;
use std::sync::Mutex;

pub struct AppState<'a> {
    pub dbus: Mutex<DBusInterface<'a>>
}
