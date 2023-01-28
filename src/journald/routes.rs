use crate::{api_errors::ApiError, journald::functions};
use actix_web::{get, http::header::ContentType, web, web::Query, HttpResponse, Responder};

#[derive(Deserialize)]
struct Info {
  lines_number: Option<usize>,
}

#[get("/unit-logs/{name}")]
async fn unit_logs(
  path: web::Path<String>,
  info: Query<Info>,
) -> Result<impl Responder, ApiError> {
  let name = path; //TODO checking if unit exists and returning appropriate http error if not
  let lines_number = info.lines_numbernumber.unwrap_or(1);

  let response_body = functions::read_n_latest_lines(&name, &lines_number);

  Ok(
    HttpResponse::Ok()
      .append_header(ContentType::json())
      .body(response_body.unwrap().to_string()),
  )
}
