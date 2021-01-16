use serde::Serialize;

#[derive(Serialize)]
pub enum Errors {
    GenderNotFound,
    SexualityNotFound,
    GenderOrSexualityNotFound,
}
