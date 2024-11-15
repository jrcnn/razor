use std::marker::PhantomData;

mod seal {
    pub trait EventCompatible {}
}

pub trait EventParamTrait<T> : seal::EventCompatible {
    fn create() -> Self;
}

pub trait NotForCopy<T> {
    fn distribute(base: &T) -> T;
}

pub struct EventSample<'a, T, U> {
    p1: EventParam<T>,
    p2: EventParam<U>,

    closures: Vec<&'a dyn Fn(&T, &U)>
}

pub struct EventParam<T> {
    _ph : PhantomData<T>
}

impl<T: Copy> EventParam<T> {
    pub fn distribute(base: &T) -> T {
        println!("copied");
        *base
    }
}

impl<T: Clone> NotForCopy<T> for EventParam<T> {
    fn distribute(base: &T) -> T {
        println!("clone called");
        base.clone()
    }
}

impl<T> seal::EventCompatible for EventParam<T> {} 

impl<T> EventParamTrait<T> for EventParam<T> {
    fn create() -> Self {
        Self {
            _ph: PhantomData
        }
    }
}
