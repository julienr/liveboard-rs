use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub value1: String,
}
