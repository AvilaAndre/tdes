use ordered_float::OrderedFloat;
use rand_distr::{Distribution, Exp, LogNormal, Normal, Uniform, Weibull};
use serde::{Deserialize, Serialize};

use crate::internal::core::Context;

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub enum DistributionWrapper {
    #[default]
    Nothing,
    Constant(f64),       // c
    Exponential(f64),    // lambda
    Gaussian(f64, f64),  // mean, std_dev
    Uniform(f64, f64),   // min, max
    Weibull(f64, f64),   // shape , scale
    LogNormal(f64, f64), // mu, sigma
}

pub fn get_value(
    ctx: &mut Context,
    distribution: DistributionWrapper,
) -> Option<OrderedFloat<f64>> {
    match distribution {
        DistributionWrapper::Nothing => Some(OrderedFloat(0.0)),
        DistributionWrapper::Constant(c) => Some(OrderedFloat(c)),
        DistributionWrapper::Exponential(lambda) => Exp::new(lambda)
            .map(|exp| OrderedFloat(exp.sample(&mut ctx.rng)))
            .ok(),
        DistributionWrapper::Gaussian(mean, std_dev) => Normal::new(mean, std_dev)
            .map(|exp| OrderedFloat(exp.sample(&mut ctx.rng)))
            .ok(),
        DistributionWrapper::Uniform(min, max) => Uniform::new(min, max)
            .map(|exp| OrderedFloat(exp.sample(&mut ctx.rng)))
            .ok(),
        DistributionWrapper::Weibull(shape, scale) => Weibull::new(scale, shape)
            .map(|exp| OrderedFloat(exp.sample(&mut ctx.rng)))
            .ok(),
        DistributionWrapper::LogNormal(mu, sigma) => LogNormal::new(mu, sigma)
            .map(|exp| OrderedFloat(exp.sample(&mut ctx.rng)))
            .ok(),
    }
}
