use faer::Mat;

use super::{
    family::{binomial, FamilyEnum},
    utils::{div_elemwise, mat_cat, mul_elementwise, sqr_elemwise, sqrt_elemwise, CatDim},
};

pub struct GeneralizedLinearModel {
    pub r_local: Mat<f64>,
    pub coefficients: Mat<f64>,
    pub family: FamilyEnum,
    pub iter: i32,
}

pub fn distributed_binomial_single_iter_n(x: Mat<f64>, y: Mat<f64>, beta: Mat<f64>) -> Mat<f64> {
    let eta = x.clone() * beta;
    let mu = binomial::linkinv(&eta);
    let dmu = binomial::mu_eta(&eta);
    let z = div_elemwise(&(eta + (y - mu.clone())), &dmu.clone());

    let w = div_elemwise(&sqr_elemwise(&dmu.clone()), &binomial::variance(&mu));

    let sqrt_w = &sqrt_elemwise(&w.clone());

    let x_tilde = &mul_elementwise(sqrt_w, &x);
    let z_tilde = &mul_elementwise(sqrt_w, &z);

    // r_local
    mat_cat(x_tilde, z_tilde, CatDim::HORIZONTAL)
        .qr()
        .R()
        .to_owned()
}
