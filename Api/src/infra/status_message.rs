

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct StatusMessage {
    pub module: String,
    pub code: String,
    pub message: String,
    pub details: HashMap<String, String>,
}

impl fmt::Display for StatusMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl actix_web::error::ResponseError for StatusMessage {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.code.as_str() {
            "Gen.I200" => actix_web::http::StatusCode::OK,
            "Gen.W400" => actix_web::http::StatusCode::BAD_REQUEST,
            "Gen.W403" => actix_web::http::StatusCode::FORBIDDEN,
            "Gen.W404" => actix_web::http::StatusCode::NOT_FOUND,
            "Gen.W500" => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            _ => {
                let code_copy = self.code.clone();
                let code_start = code_copy.find(".").unwrap_or(0) + 2;
                let mut code = code_copy[code_start..].to_string().parse::<u16>().unwrap_or(500);
                if code < 300 || code > 600 { code = 500 };
                actix_web::http::StatusCode::from_u16(code).unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code())
            .json(self)
    }
}

impl StatusMessage {
    pub fn constructor(i_code: &str, i_message: &str) -> StatusMessage {
        StatusMessage {
            module: String::from(""),
            code: String::from(i_code),
            message: String::from(i_message),
            details: HashMap::new(),
        }
    }

    pub fn make_not_found_message(message: &str) -> StatusMessage {
        StatusMessage::constructor("Gen.W404", message)
    }

    pub fn make_bad_request_message(message: &str) -> StatusMessage {
        StatusMessage::constructor("Gen.W400", message)
    }

    pub fn make_forbidden_message(message: &str) -> StatusMessage {
        StatusMessage::constructor("Gen.W403", message)
    }

    pub fn make_success_message(message: &str) -> StatusMessage {
        StatusMessage::constructor("Gen.I200", message)
    }
    pub fn make_internal_server_error_message(message: &str) -> StatusMessage {
        StatusMessage::constructor("Gen.W500", message)
    }

    pub fn not_found_int(entity: &str, id: i64) -> String {
        format!("{} {} not found", entity, id)
    }

    pub fn not_found(entity: &str, id: Option<Uuid>) -> String {
        if id.is_none() {
            format!("{} not found", entity)
        } else {
            format!("{} {} not found", entity, id.unwrap())
        }
    }

    pub fn forbidden(entity: &str, id: Option<Uuid>) -> String {
        if id.is_none() {
            format!("access to this {} not allowed", entity)
        } else {
            format!("access to {} {} not allowed", entity, id.unwrap())
        }
    }

    pub fn success(action: &str, entity: &str, id: Option<Uuid>) -> String {
        if id.is_none() {
            format!("{} of this {} successful", action, entity)
        } else {
            format!("{} of {} {} successful", action, entity, id.unwrap())
        }
    }

    pub fn internal_error(action: &str, entity: &str, id: Option<Uuid>) -> String {
        if id.is_none() {
            format!("Error {} {}", action, entity)
        } else {
            format!("Error {} {} {}", action, entity, id.unwrap())
        }
    }
}
