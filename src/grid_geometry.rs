use crate::grid::Point;

#[derive(Copy, Clone)]
pub struct GridGeometry {
    width: i32,
    height: i32,
}

impl GridGeometry {
    pub fn new(width: i32, height: i32) -> GridGeometry {
        GridGeometry { width, height }
    }
    pub fn height(&self) -> i32 {
        self.height
    }
    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn index(&self, point: &Point) -> Option<usize> {
        if !self.in_bounds(point) {
            return None;
        }
        Some(point.y as usize * self.width as usize + point.x as usize)
    }
    pub fn in_bounds(&self, point: &Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.width && point.y < self.height
    }
}
