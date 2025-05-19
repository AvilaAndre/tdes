use downcast_rs::{Downcast, impl_downcast};
use std::fmt::Debug;

use super::context::Context;

pub trait Message: Debug + Downcast {}
impl_downcast!(Message);

pub trait Timer: Debug + Downcast {
    fn fire(&self, ctx: &mut Context);
}
impl_downcast!(Timer);
