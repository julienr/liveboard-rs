use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub value1: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: String,
}

impl Clone for Circle {
    fn clone(&self) -> Self {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius,
            color: self.color.clone()
        }
    }
}
