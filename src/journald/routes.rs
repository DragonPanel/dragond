use crate::{api_errors::ApiError, journald::functions};
use actix_web::{get, http::header::ContentType, web, web::Query, HttpResponse, Responder};

#[derive(Deserialize)]
struct Info {
  lines_number: Option<usize>,
  cursor: Option<String>,
}

#[get("/unit-logs/{name}")]
async fn unit_logs(path: web::Path<String>, info: Query<Info>) -> Result<impl Responder, ApiError> {
  let info = info.into_inner();
  let name = path.to_string(); //TODO checking if unit exists and returning appropriate http error if not

  let response_body = functions::read_lines(&name, &info.lines_number, &info.cursor);

  Ok(
    HttpResponse::Ok()
      .append_header(ContentType::json())
      .body(response_body.unwrap().to_string()),
  )
}
