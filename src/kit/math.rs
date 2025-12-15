use windows::Foundation::Rect;
use windows_numerics::{Vector2, Vector3};

pub fn rect_size(rect: &Rect) -> Vector2 {
    Vector2 {
        X: rect.Width,
        Y: rect.Height,
    }
}

pub fn rect_offset(rect: &Rect) -> Vector3 {
    Vector3 {
        X: rect.X,
        Y: rect.Y,
        Z: 0.0,
    }
}

pub enum Size {
    Relative { w: f32, h: f32 },
    Absolute { w: f32, h: f32 },
}

impl Size {
    pub fn rel(w: f32, h: f32) -> Self {
        Self::Relative { w, h }
    }

    pub fn abs(w: f32, h: f32) -> Self {
        Self::Absolute { w, h }
    }
}
