use ordered_float::OrderedFloat;
use rand_distr::{Distribution, Exp, LogNormal, Normal, Uniform, Weibull};
use serde::{Deserialize, Serialize};

use crate::internal::core::Context;

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub enum DelayModifiers {
    #[default]
    Nothing,
    Constant(f64),       // c
    Exponential(f64),    // lambda
    Gaussian(f64, f64),  // mean, std_dev
    Uniform(f64, f64),   // min, max
    Weibull(f64, f64),   // shape , scale
    LogNormal(f64, f64), // mu, sigma
}

pub fn get_value(ctx: &mut Context, distribution: DelayModifiers) -> OrderedFloat<f64> {
    let val = match distribution {
        DelayModifiers::Nothing => Some(OrderedFloat(0.0)),
        DelayModifiers::Constant(c) => Some(OrderedFloat(c)),
        DelayModifiers::Exponential(lambda) => Exp::new(lambda)
            .map(|exp| OrderedFloat(exp.sample(&mut ctx.rng)))
            .ok(),

        DelayModifiers::Gaussian(mean, std_dev) => Normal::new(mean, std_dev)
            .map(|exp| OrderedFloat(exp.sample(&mut ctx.rng)))
            .ok(),
        DelayModifiers::Uniform(min, max) => Uniform::new(min, max)
            .map(|exp| OrderedFloat(exp.sample(&mut ctx.rng)))
            .ok(),
        DelayModifiers::Weibull(shape, scale) => Weibull::new(scale, shape)
            .map(|exp| OrderedFloat(exp.sample(&mut ctx.rng)))
            .ok(),
        DelayModifiers::LogNormal(mu, sigma) => LogNormal::new(mu, sigma)
            .map(|exp| OrderedFloat(exp.sample(&mut ctx.rng)))
            .ok(),
    };

    val.unwrap()
}
