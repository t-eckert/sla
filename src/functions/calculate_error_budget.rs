use azure_functions::{
    bindings::{HttpRequest, HttpResponse},
    func,
    http::Status,
};
use serde::{Deserialize, Serialize};
use serde_json::to_value;

#[derive(Serialize)]
struct Response {
    monthly: String,
}

#[func]
pub fn calculate_error_budget(req: HttpRequest) -> HttpResponse {
    match req.query_params().get("sla") {
        None => build_bad_request_response("Please pass a number to `sla` parameter."),
        Some(sla) => {
            match calculate_from_sla(sla) {
                Err(e) => build_bad_request_response(e),
                Ok(resp) => resp,
            }
        }
    }
}

fn calculate_from_sla(sla_string: &String) -> Result<Response, Err("")> {
    let sla: f64 = sla_string.parse::<f64>()?;

    Response {
        monthly: "Hello"
    }
}

fn build_bad_request_response(message: String) -> HttpResponse {
    HttpResponse::build()
        .status(Status::BadRequest)
        .body(message)
        .into()
}