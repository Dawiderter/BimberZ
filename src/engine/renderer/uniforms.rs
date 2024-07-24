use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use slotmap::{new_key_type, SlotMap};

pub const UNIFORM_SIZE: usize = 16;

#[derive(Debug, Clone)]
pub enum UniformValue {
    F32(f32),
    Vec2(glam::Vec2),
    Vec3(glam::Vec3),
}

impl UniformValue {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            UniformValue::F32(value) => bytemuck::bytes_of(value),
            UniformValue::Vec2(value) => bytemuck::bytes_of(value),
            UniformValue::Vec3(value) => bytemuck::bytes_of(value),
        }
    }

    pub fn size(&self) -> usize {
        self.as_bytes().len()
    }
}

new_key_type! { pub struct UniformKey; }

impl UniformKey {
    pub fn idx(&self) -> u32 {
        // Temporary hack for getting index of slotmap key
        self.0.as_ffi() as u32
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Uniform<T> {
    key: UniformKey,
    _marker: PhantomData<T>,
}

impl<T> Uniform<T> {
    pub fn idx(&self) -> u32 {
        self.key.idx()
    }
}

#[derive(Debug, Clone)]
pub struct Uniforms {
    values: SlotMap<UniformKey, UniformValue>,
    pub has_changed_structure: bool,
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            values: SlotMap::with_key(),
            has_changed_structure: false,
        }
    }

    pub fn bind<T: Into<UniformValue>>(&mut self, value: T) -> Uniform<T> {
        let key = self.values.insert(value.into());
        let bind = Uniform {
            key,
            _marker: PhantomData,
        };
        self.has_changed_structure = true;
        bind
    }

    pub fn unbind<T>(&mut self, handle: Uniform<T>) {
        self.values.remove(handle.key);
        self.has_changed_structure = true;
    }

    pub fn write_to_buffer(&self, buffer: &mut [u8]) {
        let mut i = 0;
        for value in self.iter() {
            let bytes = value.as_bytes();
            buffer[i..(i + bytes.len())].clone_from_slice(bytes);
            i += UNIFORM_SIZE;
        }
    }

    pub fn bytes_len(&self) -> usize {
        self.len() * UNIFORM_SIZE
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &UniformValue> + '_ {
        self.values.values()
    }

    pub fn iter_with_keys(&self) -> impl Iterator<Item = (UniformKey, &UniformValue)> {
        self.values.iter()
    }
}

impl Default for Uniforms {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<Uniform<T>> for Uniforms
where
    UniformValue: AsRef<T>,
{
    type Output = T;

    fn index(&self, index: Uniform<T>) -> &Self::Output {
        self.values[index.key].as_ref()
    }
}

impl<T> IndexMut<Uniform<T>> for Uniforms
where
    UniformValue: AsMut<T> + AsRef<T>,
{
    fn index_mut(&mut self, index: Uniform<T>) -> &mut Self::Output {
        self.values[index.key].as_mut()
    }
}

macro_rules! bindvalue_conv {
    ($($t:ty => $i:ident)*) => {
        $(
            impl From<$t> for UniformValue {
                fn from(value: $t) -> Self {
                    Self::$i(value)
                }
            }

            impl AsRef<$t> for UniformValue {
                fn as_ref(&self) -> &$t {
                    match self {
                        UniformValue::$i(val) => val,
                        _ => panic!("Wrong variant"),
                    }
                }
            }

            impl AsMut<$t> for UniformValue {
                fn as_mut(&mut self) -> &mut $t {
                    match self {
                        UniformValue::$i(val) => val,
                        _ => panic!("Wrong variant"),
                    }
                }
            }
        )*
    }
}

bindvalue_conv!(
    f32 => F32
    glam::Vec2 => Vec2
    glam::Vec3 => Vec3
);
