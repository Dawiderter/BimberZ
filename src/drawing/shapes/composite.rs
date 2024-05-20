use std::ops::{Add, Mul, Neg, Sub};

use crate::math::{
    rect::{rect, Rect},
    vector::{vec2, Vec2},
};

use super::{circle::Circle, rect::RectShape, shape::{Shape, Fragment}, transf::TransformedShape, coloring::{ColoredShape, StrokedShape}};

#[derive(Debug, Clone, Copy)]
pub struct ShapeUnion<A, B> {
    pub a: A,
    pub b: B,
}

impl<A: Shape, B: Shape> Shape for ShapeUnion<A, B> {
    fn frag(&self, v: Vec2<i32>) -> Fragment {
        let a_frag = self.a.frag(v);
        let b_frag = self.b.frag(v);
        if a_frag.dist <= b_frag.dist {
            a_frag
        } else {
            b_frag
        }
    }

    fn bounding_box(&self) -> Rect<i32> {
        self.a.bounding_box().combine(self.b.bounding_box())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ShapeIntersect<A, B> {
    pub a: A,
    pub b: B,
}

impl<A: Shape, B: Shape> Shape for ShapeIntersect<A, B> {
    fn frag(&self, v: Vec2<i32>) -> Fragment {
        let a_frag = self.a.frag(v);
        let b_frag = self.b.frag(v);
        if a_frag.dist >= b_frag.dist {
            a_frag
        } else {
            b_frag
        }
    }

    fn bounding_box(&self) -> Rect<i32> {
        self.a.bounding_box().intersection(self.b.bounding_box())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ShapeDiff<A, B> {
    pub a: A,
    pub b: B,
}

impl<A: Shape, B: Shape> Shape for ShapeDiff<A, B> {
    fn frag(&self, v: Vec2<i32>) -> Fragment {
        let a_frag = self.a.frag(v);
        let b_frag = self.b.frag(v);
        let b_inv_frag = Fragment::new(-b_frag.dist, b_frag.color);
        if a_frag.dist >= b_inv_frag.dist {
            a_frag
        } else {
            b_inv_frag
        }
    }

    fn bounding_box(&self) -> Rect<i32> {
        self.a.bounding_box()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ShapeInvert<A> {
    pub a: A,
}

impl<A: Shape> Shape for ShapeInvert<A> {
    fn frag(&self, v: Vec2<i32>) -> Fragment {
        let child_frag = self.a.frag(v);
        Fragment::new(-child_frag.dist, child_frag.color)
    }

    fn bounding_box(&self) -> Rect<i32> {
        self.a.bounding_box()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ShapeExtend<A> {
    pub a: A,
    pub r: i32,
}

impl<A: Shape> Shape for ShapeExtend<A> {
    fn frag(&self, v: Vec2<i32>) -> Fragment {
        let child_frag = self.a.frag(v);
        Fragment::new(child_frag.dist - self.r as f32, child_frag.color)   
    }

    fn bounding_box(&self) -> Rect<i32> {
        let bb = self.a.bounding_box();
        rect(
            bb.top_left - vec2(self.r, self.r),
            bb.bot_right + vec2(self.r, self.r),
        )
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

        impl<R : Shape> Add<R> for $t {
            type Output = ShapeUnion<Self,R>;

            fn add(self, rhs: R) -> Self::Output {
                Self::Output { a: self, b: rhs}
            }
        }

        impl<R : Shape> Mul<R> for $t {
            type Output = ShapeIntersect<Self,R>;

            fn mul(self, rhs: R) -> Self::Output {
                Self::Output { a: self, b: rhs}
            }
        }

        impl Add<i32> for $t {
            type Output = ShapeExtend<Self>;

            fn add(self, rhs: i32) -> Self::Output {
                Self::Output { a: self, r: rhs }
            }
        }
        impl Sub<i32> for $t {
            type Output = ShapeExtend<Self>;

            fn sub(self, rhs: i32) -> Self::Output {
                Self::Output { a: self, r: -rhs }
            }
        }
        impl Neg for $t {
            type Output = ShapeInvert<Self>;

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

        impl<R : Shape, $($i : Shape),*> Add<R> for $t {
            type Output = ShapeUnion<Self,R>;

            fn add(self, rhs: R) -> Self::Output {
                Self::Output { a: self, b: rhs}
            }
        }

        impl<R : Shape, $($i : Shape),*> Mul<R> for $t {
            type Output = ShapeIntersect<Self,R>;

            fn mul(self, rhs: R) -> Self::Output {
                Self::Output { a: self, b: rhs}
            }
        }

        impl<$($i : Shape),*> Add<i32> for $t {
            type Output = ShapeExtend<Self>;

            fn add(self, rhs: i32) -> Self::Output {
                Self::Output { a: self, r: rhs }
            }
        }
        impl<$($i : Shape),*> Sub<i32> for $t {
            type Output = ShapeExtend<Self>;

            fn sub(self, rhs: i32) -> Self::Output {
                Self::Output { a: self, r: -rhs }
            }
        }
        impl<$($i : Shape),*> Neg for $t {
            type Output = ShapeInvert<Self>;

            fn neg(self) -> Self::Output {
                Self::Output { a: self }
            }
        }
    };
}

impl_ops!(Circle);
impl_ops!(RectShape);
impl_ops!(TransformedShape<A>, A);
impl_ops!(ColoredShape<A>, A);
impl_ops!(StrokedShape<A>, A);
impl_ops!(ShapeInvert<A>, A);
impl_ops!(ShapeExtend<A>, A);
impl_ops!(ShapeDiff<A,B>, A, B);
impl_ops!(ShapeIntersect<A,B>, A, B);
impl_ops!(ShapeUnion<A,B>, A, B);
