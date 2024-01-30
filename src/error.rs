use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorReason {
    pub status: u16,
    pub reason: Vec<String>,
}

impl ErrorReason {
    pub fn new(status: u16, reason: Vec<String>) -> Self {
        Self { status, reason }
    }
    pub fn add_bad_request(reason: String) -> Self {
        Self {
            status: 400,
            reason: vec![reason],
        }
    }

    pub fn bad_request(reason: String) -> (StatusCode, Result<String, Json<ErrorReason>>) {
        (
            StatusCode::BAD_REQUEST,
            Err(Json(Self {
                status: 400,
                reason: vec![reason],
            })),
        )
    }
    pub fn failed(reason: String) -> Result<String, Json<ErrorReason>> {
        Err(Json(Self {
            status: 400,
            reason: vec![reason],
        }))
    }
}
