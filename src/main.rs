mod systemd;
mod api_errors;
mod dbus_interface;
mod app_state;

use std::sync::Mutex;
use crate::app_state::AppState;
use actix_web::{web, App, HttpServer};
use dbus_interface::DBusInterface;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = AppState { dbus: Mutex::new(DBusInterface::new()) };
    let app_data = web::Data::new(state);

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::clone(&app_data))
        .service(
            web::scope("/systemd")
            .service(systemd::routes::load_unit)
            .service(systemd::routes::list_units)
        )
    })
    .bind(("0.0.0.0", 1337))?
    .run()
    .await
}
