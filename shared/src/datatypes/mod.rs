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
    pub color: Color,
}

impl Clone for Circle {
    fn clone(&self) -> Self {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius,
            color: self.color,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn hex_color(&self) -> String {
        return format!("#{}", hex::encode([self.r, self.g, self.b]));
    }
}
