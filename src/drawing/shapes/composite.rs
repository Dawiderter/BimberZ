use super::shape::Shape;

pub struct ShapeUnion<A,B> {
    pub a: A,
    pub b: B,
}

impl<A: Shape, B: Shape> Shape for ShapeUnion<A,B> {
    fn dist(&self, x: i32, y: i32) -> f32 {
        self.a.dist(x, y).min(self.b.dist(x, y)) 
    }

    fn bounding_box(&self) -> ((i32, i32), (i32, i32)) {
        ((i32::MIN, i32::MIN), (i32::MAX, i32::MAX))
    }
}

pub struct ShapeDiff<A,B> {
    pub a: A,
    pub b: B,
}

impl<A: Shape, B: Shape> Shape for ShapeDiff<A,B> {
    fn dist(&self, x: i32, y: i32) -> f32 {
        self.a.dist(x, y).max(-self.b.dist(x, y)) 
    }

    fn bounding_box(&self) -> ((i32, i32), (i32, i32)) {
        ((i32::MIN, i32::MIN), (i32::MAX, i32::MAX))
    }
}