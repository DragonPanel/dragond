use crate::{api_errors::ApiError, systemd::functions, AppState};
use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder};

#[get("/load-unit/{name}")]
async fn load_unit(
  state: web::Data<AppState<'static>>,
  path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
  let name = path;
  let dbus = state.dbus.lock().unwrap();
  let unit = functions::load_unit_data(&dbus, &name)?;

  let serialized = serde_json::to_string(&unit).unwrap_or("{}".to_owned());

  Ok(
    HttpResponse::Ok()
      .append_header(ContentType::json())
      .body(serialized),
  )
}

#[get("/list-units")]
async fn list_units(state: web::Data<AppState<'static>>) -> Result<impl Responder, ApiError> {
  let dbus = state.dbus.lock().unwrap();
  let units = functions::list_units(&dbus)?;

  let serialized = serde_json::to_string(&units).unwrap_or("{}".to_owned());

  Ok(
    HttpResponse::Ok()
      .append_header(ContentType::json())
      .body(serialized),
  )
}
