use downcast_rs::{Downcast, impl_downcast};
use std::fmt::Debug;

pub trait MessageClone {
    fn clone_box(&self) -> Box<dyn Message>;
}

impl<T> MessageClone for T
where
    T: 'static + Message + Clone,
{
    fn clone_box(&self) -> Box<dyn Message> {
        Box::new(self.clone())
    }
}

pub trait Message: Debug + Downcast + MessageClone {
    fn size_bits(&self) -> u64 {
        self.size_bytes() * 8 // Default: convert bytes to bits
    }

    fn size_bytes(&self) -> u64;
}
impl_downcast!(Message);
