use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

pub const fn ivec2(x: i32, y: i32) -> IVec2 {
    IVec2 { x, y }
}

impl IVec2 {
    pub fn manhattan(self, other: IVec2) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for IVec2 {
    type Output = IVec2;
    fn add(self, o: IVec2) -> IVec2 {
        ivec2(self.x + o.x, self.y + o.y)
    }
}

impl Sub for IVec2 {
    type Output = IVec2;
    fn sub(self, o: IVec2) -> IVec2 {
        ivec2(self.x - o.x, self.y - o.y)
    }
}

impl From<(i32, i32)> for IVec2 {
    fn from((x, y): (i32, i32)) -> Self {
        ivec2(x, y)
    }
}
