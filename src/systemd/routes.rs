use crate::{
  api_errors::ApiError,
  AppState, systemd::functions::load_unit_data,
};
use actix_web::{get, web, HttpResponse, Responder, http::header::ContentType};

#[get("/load-unit/{name}")]
async fn load_unit(
  state: web::Data<AppState<'static>>,
  path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
  let name = path;
  let dbus = state.dbus.lock().unwrap();
  let unit = load_unit_data(&dbus, &name)?;

  let serialized = serde_json::to_string(&unit).unwrap_or("{}".to_owned());

  Ok(
    HttpResponse::Ok()
      .append_header(ContentType::json())
      .body(serialized)
  )
}
