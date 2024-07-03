#[derive(Debug)]
pub enum Uniform {
    F32(f32),
    U32(u32),
    Vec2f(glam::Vec2),
    Vec3f(glam::Vec3),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UniformType {
    F32,
    U32,
    Vec2f,
    Vec3f,
}

macro_rules! impl_uniform_conversions {
    ($( $i:ident($t:ty) ),*) => {
        $(
            impl From<$t> for Uniform {
                fn from(value: $t) -> Self {
                    Self::$i(value)
                }
            }
            impl AsRef<$t> for Uniform {
                fn as_ref(&self) -> &$t {
                    let Self::$i(x) = self else { panic!("Incorrect type") };
                    x
                }
            }
            impl AsMut<$t> for Uniform {
                fn as_mut(&mut self) -> &mut $t {
                    let Self::$i(x) = self else { panic!("Incorrect type") };
                    x
                }
            }
        )*
    };
}

impl_uniform_conversions!(
    F32(f32),
    U32(u32),
    Vec2f(glam::Vec2),
    Vec3f(glam::Vec3)
);

impl UniformType {
    /// Size of a type in bytes
    pub fn size(&self) -> u8 {
        match self {
            UniformType::F32 => 4,
            UniformType::U32 => 4,
            UniformType::Vec2f => 8,
            UniformType::Vec3f => 12,
        }
    }
}

pub trait AsUniformType {
    fn as_uniform_type() -> UniformType;
}

macro_rules! impl_as_uniform_type {
    ($( $t:ty => $i:ident ),*) => {
        $(
            impl AsUniformType for $t {
                fn as_uniform_type() -> UniformType {
                    UniformType::$i
                }
            }
        )*
    };
}

impl_as_uniform_type!(
    f32 => F32,
    u32 => U32,
    glam::Vec2 => Vec2f,
    glam::Vec3 => Vec3f
);
