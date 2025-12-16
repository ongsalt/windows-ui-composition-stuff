use windows::Foundation::Rect;
use windows_numerics::{Vector2, Vector3};

// TODO: make this proper extension
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

#[derive(Debug, Clone, Copy)]
pub struct Constraints {
    pub max_w: f32,
    pub min_w: f32,
    pub max_h: f32,
    pub min_h: f32,
}

impl Constraints {
    pub fn from_size(size: Vector2) -> Self {
        Self {
            max_w: size.X,
            min_w: 0.0,
            max_h: size.Y,
            min_h: 0.0,
        }
    }

    pub fn coerce(&self, size: Vector2) -> Vector2 {
        Vector2::new(
            size.X.clamp(self.min_w, self.max_w),
            size.Y.clamp(self.min_h, self.max_h),
        )
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

// I shuold probably use layout engine library
#[derive(Debug, Clone, Copy)]
pub enum SizePreference {
    Default,
    FillAvailable,
    Fixed(f32),
}

impl SizePreference {
    pub fn compute(&self, size: f32, min: f32, max: f32) -> f32 {
        match self {
            SizePreference::Default => size.clamp(min, max),
            SizePreference::FillAvailable => max,
            SizePreference::Fixed(i) => *i,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Margin {
    pub top: f32,
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
}

impl Margin {
    pub fn all(size: f32) -> Self {
        Self {
            top: size,
            left: size,
            bottom: size,
            right: size,
        }
    }

    pub fn zero() -> Self {
        Self::all(0.0)
    }

    pub fn xy(x: f32, y: f32) -> Self {
        Self {
            top: y,
            bottom: y,
            left: x,
            right: x,
        }
    }

    pub fn y(size: f32) -> Self {
        Self {
            top: size,
            bottom: size,
            left: 0.0,
            right: 0.0,
        }
    }

    pub fn x(size: f32) -> Self {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: size,
            right: size,
        }
    }

    pub fn w(&self) -> f32 {
        self.left + self.right
    }

    pub fn h(&self) -> f32 {
        self.top + self.bottom
    }

    pub fn new(top: f32, left: f32, bottom: f32, right: f32) -> Self {
        Self {
            top,
            left,
            bottom,
            right,
        }
    }

    pub fn top(size: f32) -> Self {
        Self {
            top: size,
            left: 0.0,
            bottom: 0.0,
            right: 0.0,
        }
    }

    pub fn left(size: f32) -> Self {
        Self {
            top: 0.0,
            left: size,
            bottom: 0.0,
            right: 0.0,
        }
    }

    pub fn bottom(size: f32) -> Self {
        Self {
            top: 0.0,
            left: 0.0,
            bottom: size,
            right: 0.0,
        }
    }

    pub fn right(size: f32) -> Self {
        Self {
            top: 0.0,
            left: 0.0,
            bottom: 0.0,
            right: size,
        }
    }

    pub fn with_left(mut self, left: f32) -> Self {
        self.left = left;
        self
    }

    pub fn with_right(mut self, right: f32) -> Self {
        self.right = right;
        self
    }

    pub fn with_top(mut self, top: f32) -> Self {
        self.top = top;
        self
    }

    pub fn with_bottom(mut self, bottom: f32) -> Self {
        self.bottom = bottom;
        self
    }

    pub fn with_x(mut self, x: f32) -> Self {
        self.left = x;
        self.right = x;
        self
    }

    pub fn with_y(mut self, y: f32) -> Self {
        self.top = y;
        self.bottom = y;
        self
    }
}

impl Default for Margin {
    fn default() -> Self {
        Self::all(0.0)
    }
}
