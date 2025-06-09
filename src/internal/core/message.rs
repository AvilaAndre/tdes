use downcast_rs::{Downcast, impl_downcast};
use std::fmt::Debug;

pub trait Message: Debug + Downcast {
    fn size_bits(&self) -> u64 {
        self.size_bytes() * 8 // Default: convert bytes to bits
    }

    fn size_bytes(&self) -> u64;
}
impl_downcast!(Message);
