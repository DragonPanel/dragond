use crate::{api_errors::ApiError, journald::functions, AppState};
use actix_web::{get, http::header::ContentType, web, web::Query, HttpResponse, Responder};
use std::process::Command;

#[derive(Deserialize)]
struct Info {
  number: isize,
}

#[get("/read-latest/{name}")]
async fn read_latest(
  path: web::Path<String>,
  info: Query<Info>,
) -> Result<impl Responder, ApiError> {
  let name = path;
  let lines_num = info.number;
  debug!(target: "app_events", "input params: unit name {}, num of lines {}", &name, &lines_num);

  //TODO Extract this into functions file
  let output = Command::new("journalctl")
    .arg("--no-pager")
    .arg("-r")
    .args(["-u", &name])
    .args(["-o", "json"])
    .args(["-n", &lines_num.to_string()])
    .output()
    .expect("failure in executing journalctl");

  info!("read_latest {}", &output.status);
  // let serialized = serde_json::to_string(&output.stdout).unwrap_or("{}".to_owned());

  Ok(
    HttpResponse::Ok()
      .append_header(ContentType::json())
      .body(output.stdout),
  )
}
