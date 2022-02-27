use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub name: String,
}