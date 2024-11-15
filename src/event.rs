use std::marker::PhantomData;

mod seal {
    pub trait Seal {}
}

pub trait EventCompatible<T> : seal::Seal {
    fn create() -> Self;
}

impl<T> seal::Seal for EventParam<T> {} 

impl<T> EventCompatible<T> for EventParam<T> {
    fn create() -> Self {
        Self {
            _ph: PhantomData
        }
    }
}

pub trait CloneDistribute<T> {
    fn distribute(base: &T) -> T;
}

pub struct EventParam<T> {
    _ph : PhantomData<T>
}

impl<T: Copy> EventParam<T> {
    pub fn distribute(base: &T) -> T {
        *base
    }
}

impl<T: Clone> CloneDistribute<T> for EventParam<T> {
    fn distribute(base: &T) -> T {
        base.clone()
    }
}
