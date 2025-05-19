use std::fmt::Debug;
use downcast_rs::{impl_downcast, Downcast};

pub trait Message: Debug + Downcast {}
impl_downcast!(Message);
