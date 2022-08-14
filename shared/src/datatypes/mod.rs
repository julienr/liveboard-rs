use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SocketMessage {
    Circle(Circle),
    Pointer(PointerPosition),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerPosition {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub color: Color,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub name: String,
    pub id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CreateBoardRequest {
    pub name: String,
}
