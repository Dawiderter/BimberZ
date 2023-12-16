use std::ops::{Sub, BitOr, BitAnd, Add, Neg};

use crate::math::{vector::{Vec2, vec2}, rect::{Rect, rect}};

use super::{shape::Shape, circle::Circle, rect::RectShape, transf::TransformedShape};

#[derive(Debug, Clone, Copy)]
pub struct ShapeUnion<A,B> {
    pub a: A,
    pub b: B,
}

impl<A: Shape, B: Shape> Shape for ShapeUnion<A,B> {
    fn dist(&self, v: Vec2<i32>) -> f32 {
        self.a.dist(v).min(self.b.dist(v)) 
    }

    fn bounding_box(&self) -> Rect<i32> {
        self.a.bounding_box().combine(self.b.bounding_box())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ShapeInter<A,B> {
    pub a: A,
    pub b: B,
}

impl<A: Shape, B: Shape> Shape for ShapeInter<A,B> {
    fn dist(&self, v: Vec2<i32>) -> f32 {
        self.a.dist(v).max(self.b.dist(v)) 
    }

    fn bounding_box(&self) -> Rect<i32> {
        self.a.bounding_box().intersection(self.b.bounding_box())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ShapeDiff<A,B> {
    pub a: A,
    pub b: B,
}

impl<A: Shape, B: Shape> Shape for ShapeDiff<A,B> {
    fn dist(&self, v: Vec2<i32>) -> f32 {
        self.a.dist(v).max(-self.b.dist(v)) 
    }

    fn bounding_box(&self) -> Rect<i32> {
        self.a.bounding_box()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Invert<A> {
    pub a: A
}

impl<A: Shape> Shape for Invert<A> {
    fn dist(&self, v: Vec2<i32>) -> f32 {
        -self.a.dist(v)
    }

    fn bounding_box(&self) -> Rect<i32> {
        self.a.bounding_box()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Extend<A> {
    pub a: A,
    pub r: i32,
}

impl<A: Shape> Shape for Extend<A> {
    fn dist(&self, v: Vec2<i32>) -> f32 {
        self.a.dist(v) - self.r as f32
    }

    fn bounding_box(&self) -> Rect<i32> {
        let bb = self.a.bounding_box();
        rect(bb.top_left - vec2(self.r, self.r), bb.bot_right + vec2(self.r, self.r))
    }
}

macro_rules! impl_ops {
    ($t:ty) => {
        impl<R : Shape> Sub<R> for $t {
            type Output = ShapeDiff<Self,R>;
        
            fn sub(self, rhs: R) -> Self::Output {
                Self::Output { a: self, b: rhs }
            }
        }
        
        impl<R : Shape> BitOr<R> for $t {
            type Output = ShapeUnion<Self,R>;
        
            fn bitor(self, rhs: R) -> Self::Output {
                Self::Output { a: self, b: rhs} 
            }    
        }

        impl<R : Shape> BitAnd<R> for $t {
            type Output = ShapeInter<Self,R>;
        
            fn bitand(self, rhs: R) -> Self::Output {
                Self::Output { a: self, b: rhs} 
            }    
        }

        impl Add<i32> for $t {
            type Output = Extend<Self>;
        
            fn add(self, rhs: i32) -> Self::Output {
                Self::Output { a: self, r: rhs }
            }
        }
        impl Sub<i32> for $t {
            type Output = Extend<Self>;
        
            fn sub(self, rhs: i32) -> Self::Output {
                Self::Output { a: self, r: -rhs }
            }
        }
        impl Neg for $t {
            type Output = Invert<Self>;
        
            fn neg(self) -> Self::Output {
                Self::Output { a: self }
            }
        }
    };
    ($t:ty, $( $i:ident ),*) => {
        impl<R : Shape, $($i : Shape),*> Sub<R> for $t {
            type Output = ShapeDiff<Self,R>;
        
            fn sub(self, rhs: R) -> Self::Output {
                Self::Output { a: self, b: rhs }
            }
        }
        
        impl<R : Shape, $($i : Shape),*> BitOr<R> for $t {
            type Output = ShapeUnion<Self,R>;
        
            fn bitor(self, rhs: R) -> Self::Output {
                Self::Output { a: self, b: rhs} 
            }    
        }

        impl<R : Shape, $($i : Shape),*> BitAnd<R> for $t {
            type Output = ShapeInter<Self,R>;
        
            fn bitand(self, rhs: R) -> Self::Output {
                Self::Output { a: self, b: rhs} 
            }    
        }

        impl<$($i : Shape),*> Add<i32> for $t {
            type Output = Extend<Self>;
        
            fn add(self, rhs: i32) -> Self::Output {
                Self::Output { a: self, r: rhs }
            }
        }
        impl<$($i : Shape),*> Sub<i32> for $t {
            type Output = Extend<Self>;
        
            fn sub(self, rhs: i32) -> Self::Output {
                Self::Output { a: self, r: -rhs }
            }
        }
        impl<$($i : Shape),*> Neg for $t {
            type Output = Invert<Self>;
        
            fn neg(self) -> Self::Output {
                Self::Output { a: self }
            }
        }
    };
}

impl_ops!(Circle);
impl_ops!(RectShape);
impl_ops!(Invert<A>, A);
impl_ops!(Extend<A>, A);
impl_ops!(TransformedShape<A>, A);
impl_ops!(ShapeDiff<A,B>, A, B);
impl_ops!(ShapeInter<A,B>, A, B);
impl_ops!(ShapeUnion<A,B>, A, B);
