use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
enum Type {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Response {
    pub(crate) message: String,
    type_: Type,
}

impl Response {
    pub fn success(message: String) -> Self {
        Self {
            message,
            type_: Type::Success,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            message,
            type_: Type::Error,
        }
    }
}
