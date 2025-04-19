#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

impl IVec2 {
    pub fn new(x: i32, y: i32) -> Self {
        IVec2 { x, y }
    }

    pub fn add(&self, other: &IVec2) -> IVec2 {
        IVec2::new(self.x + other.x, self.y + other.y)
    }

    pub fn sub(&self, other: &IVec2) -> IVec2 {
        IVec2::new(self.x - other.x, self.y - other.y)
    }

    pub fn mul(&self, scalar: i32) -> IVec2 {
        IVec2::new(self.x * scalar, self.y * scalar)
    }
}
