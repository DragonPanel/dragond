use actix_web::{
  http::{header::ContentType, StatusCode},
  HttpResponse, ResponseError,
};
use dbus::Error as DBusError;
use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Debug, Display, Error)]
pub enum ApiError {
  DBus(DBusError),
}

#[derive(Serialize)]
pub struct ErrorType {
  namespace: String,
  inner: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorData {
  status: u16,
  error_type: ErrorType,
  message: Option<String>,
}

impl ApiError {
  pub fn error_data(&self) -> ApiErrorData {
    match &self {
      ApiError::DBus(err) => err.to_error_data(),
      #[allow(unreachable_patterns)]
      _ => ApiErrorData {
        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        error_type: ErrorType {
          namespace: "Unknown".to_owned(),
          inner: None,
        },
        message: None,
      },
    }
  }
}

impl From<DBusError> for ApiError {
  fn from(err: DBusError) -> Self {
    Self::DBus(err)
  }
}

impl ResponseError for ApiError {
  fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
    let error_data = self.error_data();
    let serialized = serde_json::to_string(&error_data).unwrap_or("{}".to_owned());

    HttpResponse::build(
      StatusCode::from_u16(error_data.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
    )
    .insert_header(ContentType::json())
    .body(serialized)
  }
}

trait ToErrorData {
  fn to_error_data(&self) -> ApiErrorData;
  fn unknown(&self) -> ApiErrorData {
    ApiErrorData {
      status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
      error_type: ErrorType {
        namespace: "Unknown".to_owned(),
        inner: None,
      },
      message: None,
    }
  }
}

impl ToErrorData for DBusError {
  fn to_error_data(&self) -> ApiErrorData {
    match self.name() {
      Some(name) => match name {
        "org.freedesktop.DBus.Error.InvalidArgs" => ApiErrorData {
          status: StatusCode::BAD_REQUEST.as_u16(),
          error_type: ErrorType {
            namespace: "DBus".to_owned(),
            inner: Some(name.to_owned()),
          },
          message: self.message().map(str::to_string),
        },
        _ => self.unknown(),
      },
      None => self.unknown(),
    }
  }

  fn unknown(&self) -> ApiErrorData {
    ApiErrorData {
      status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
      error_type: ErrorType {
        namespace: "DBus".to_owned(),
        inner: self.name().map(str::to_string),
      },
      message: self.message().map(str::to_string),
    }
  }
}
