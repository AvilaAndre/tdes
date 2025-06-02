use downcast_rs::{Downcast, impl_downcast};
use std::fmt::Debug;

pub trait Message: Debug + Downcast {}
impl_downcast!(Message);
