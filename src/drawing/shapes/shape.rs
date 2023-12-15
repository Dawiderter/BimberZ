pub trait Shape {
    fn dist(&self, x: i32, y: i32) -> f32;
    fn bounding_box(&self) -> ((i32, i32), (i32, i32));
}