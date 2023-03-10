#[macro_use]
extern crate serde_derive;

mod api_errors;
mod app_state;
mod dbus_interface;
mod journald;
mod systemd;

use crate::app_state::AppState;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dbus_interface::DBusInterface;
use env_logger::Env;
use std::sync::Mutex;

#[macro_use]
extern crate log;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(Env::default().default_filter_or("info"));

  let state = AppState {
    dbus: Mutex::new(DBusInterface::new()),
  };
  let app_data = web::Data::new(state);

  let host = "0.0.0.0";
  let port = 4444;

  let server = HttpServer::new(move || {
    App::new()
      .app_data(web::Data::clone(&app_data))
      .wrap(Logger::new(
        "%a \"%r\" %s %bB \"%{Referer}i\" \"%{User-Agent}i\" %Ts",
      ))
      .service(
        web::scope("/systemd")
          .service(systemd::routes::load_unit)
          .service(systemd::routes::list_units),
      )
      .service(web::scope("/journald").service(journald::routes::unit_logs))
  })
  .bind((host, port))?;
  info!("Server bound on {}:{}", host, port);

  server.run().await
}
