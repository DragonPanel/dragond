use crate::systemd::dbus::{
  manager::OrgFreedesktopSystemd1Manager, service::OrgFreedesktopSystemd1Service,
  unit::OrgFreedesktopSystemd1Unit,
};
use dbus::blocking::{Connection, Proxy};
use std::{ops::Deref, time::Duration};

static SYSTEMD_DESTINATION: &str = "org.freedesktop.systemd1";
static SYSTEMD_MANAGER_PATH: &str = "/org/freedesktop/systemd1";

pub struct DBusInterface<'a> {
  _connection: Box<Connection>,
  systemd_manager: Proxy<'a, &'a Connection>,
}

// So I can send DBusInterface instance to different threads in a Mutex
unsafe impl Send for DBusInterface<'_> {}

impl<'b> DBusInterface<'b> {
  pub fn new<'a>() -> DBusInterface<'a> {
    // Okey, so I keep conn in box, coz box puts it's content in a fixed memory
    // location and won't change it
    let conn = Box::new(Connection::new_system().unwrap());

    // Create proxy will use it's pointer to create proxy which borrows it.
    let systemd_proxy = Self::create_proxy(
      &conn,
      SYSTEMD_DESTINATION,
      SYSTEMD_MANAGER_PATH,
      Duration::from_secs(5),
    );

    // And I put connection to struct, so it won't get deleted from memory
    // that would lead to my favourite "Segmentation fault (core dumped)"
    return DBusInterface {
      _connection: conn,
      systemd_manager: systemd_proxy,
    };
  }

  pub fn systemd_manager(&self) -> &(impl OrgFreedesktopSystemd1Manager + 'b) {
    &self.systemd_manager
  }

  fn create_proxy<'a>(
    connection: &Box<Connection>,
    dest: &'a str,
    path: &'a str,
    timeout: Duration,
  ) -> Proxy<'a, &'a Connection> {
    let proxy = unsafe {
      // I can take a pointer to Connection after dereferencing Box
      let conn_ptr: *const Connection = connection.deref();
      // Then I create proxy with it, which borrows connection
      (*conn_ptr).with_proxy(dest, path, timeout)
    };
    return proxy;
  }

  pub fn systemd_proxy_for_path(&self, path: &'b str) -> Proxy<'b, &'b Connection> {
    Self::create_proxy(
      &self._connection,
      SYSTEMD_DESTINATION,
      path,
      Duration::from_secs(5),
    )
  }

  pub fn systemd_unit(&self, path: &'b str) -> impl OrgFreedesktopSystemd1Unit + 'b {
    self.systemd_proxy_for_path(&path)
  }

  pub fn systemd_service(&self, path: &'b str) -> impl OrgFreedesktopSystemd1Service + 'b {
    self.systemd_proxy_for_path(&path)
  }
}
