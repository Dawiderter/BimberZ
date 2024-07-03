use std::{marker::PhantomData, ops::{Deref, DerefMut}, sync::{Arc, RwLock}};

pub(crate) const BIND_BYTES : u64 = 16;

type BindingBuffer = Arc<RwLock<Vec<[u8; BIND_BYTES as usize]>>>;

#[derive(Debug, Clone)]
pub struct Bind<T> {
    bindings: BindingBuffer,
    pub slot: usize,
    marker: PhantomData<T>,
}

#[derive(Debug)]
pub struct BindValue<'bind, T : bytemuck::AnyBitPattern + bytemuck::NoUninit> {
    bind: &'bind Bind<T>,
    value: T,
}

#[derive(Debug)]
pub struct Bindings {
    pub(crate) bindings: BindingBuffer,
    pub(crate) need_rebinding: bool,
}

impl<T : bytemuck::AnyBitPattern + bytemuck::NoUninit> Bind<T> {
    pub fn set(&self, value : T) {
        let buff = &[value];
        let slice = &mut self.bindings.write().unwrap()[self.slot][..std::mem::size_of::<T>()];
        slice.copy_from_slice(bytemuck::cast_slice(buff));
    }
    pub fn get(&self) -> T {
        let bytes = &self.bindings.read().unwrap()[self.slot][..std::mem::size_of::<T>()];
        bytemuck::cast_slice(bytes)[0]
    } 
    pub fn val(&self) -> BindValue<'_,T> {
        BindValue { bind: self, value: self.get() }
    }
}

impl<'bind, T : bytemuck::AnyBitPattern + bytemuck::NoUninit> Deref for BindValue<'bind, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'bind, T : bytemuck::AnyBitPattern + bytemuck::NoUninit> DerefMut for BindValue<'bind, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<'bind, T : bytemuck::AnyBitPattern + bytemuck::NoUninit> Drop for BindValue<'bind, T> {
    fn drop(&mut self) {
        self.bind.set(self.value)
    }
}


impl Bindings {
    pub fn new() -> Self {
        Self { bindings: Arc::new(RwLock::new(Vec::new())), need_rebinding: true }
    }

    pub fn bind<T>(&mut self, slot: usize) -> Bind<T> {
        let mut bindings = self.bindings.write().unwrap();

        if bindings.len() <= slot {
            bindings.resize(slot + 1, [0; BIND_BYTES as usize]);
            self.need_rebinding = true;
        }

        Bind { bindings: self.bindings.clone(), slot, marker: PhantomData }
    } 

    pub fn used_slots(&self) -> usize {
        self.bindings.read().unwrap().len()
    }
}

impl Default for Bindings {
    fn default() -> Self {
        Self::new()
    }
}

