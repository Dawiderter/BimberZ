use super::shape::Shape;

pub struct Circle {
    pub center: (i32, i32),
    pub radius: i32,
}

impl Shape for Circle {
    fn dist(&self, x: i32, y: i32) -> f32 {
        let (c_x,c_y) = self.center;
        let r = self.radius;
        let d = (x - c_x).pow(2) + (y - c_y).pow(2);
        (d as f32).sqrt() - r as f32
    }

    fn bounding_box(&self) -> ((i32, i32), (i32, i32)) {
        let (c_x,c_y) = self.center;
        let r = self.radius;
        ((c_x - r, c_y - r), (c_x + r, c_y + r))
    }
}