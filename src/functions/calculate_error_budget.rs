use azure_functions::{
    bindings::{HttpRequest, HttpResponse},
    func,
};

#[func]
pub fn calculate_error_budget(req: HttpRequest) -> HttpResponse {
    "Hello from Rust!".into()
}
