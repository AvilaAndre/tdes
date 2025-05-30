use faer::{
    Mat, get_global_parallelism,
    linalg::{solvers::DenseSolveCore, triangular_solve::solve_upper_triangular_in_place},
};

use super::{
    family::{FamilyEnum, binomial},
    utils::{
        CatDim, mat_cat, mat_diag, mat_div_elemwise, mat_sqr_elemwise, mat_sqrt_elemwise,
        mul_elementwise,
    },
};

pub const DEFAULT_MAXIT: usize = 25;
pub const DEFAULT_TOL: f64 = 1.0e-10;

pub struct GeneralizedLinearModel {
    pub r_local: Mat<f64>,
    pub coefficients: Mat<f64>,
    pub family: FamilyEnum,
    pub iter: usize,
}

fn vcov(r_local: Mat<f64>, family: FamilyEnum, total_nrow: usize) -> Mat<f64> {
    let (n, m) = r_local.shape();

    let r = r_local.submatrix(0, 0, n - 1, m - 1).to_owned();
    let rss = r_local[(n - 1, m - 1)];

    let inv_r = (r.clone().transpose() * r).partial_piv_lu().inverse();

    let dispersion = match family {
        FamilyEnum::BINOMIAL => 1.0,
        FamilyEnum::GAUSSIAN => rss * rss / (total_nrow - m) as f64,
    };

    inv_r * dispersion
}

fn ols_n(r_xy_or_xy: Mat<f64>) -> (Mat<f64>, Mat<f64>) {
    let qr = r_xy_or_xy.qr();

    let r_s = qr.R();

    let (n, m) = r_s.shape();
    let r = r_s.submatrix(0, 0, n - 1, m - 1).to_owned();
    let mut theta = r_s.submatrix(0, m - 1, n - 1, 1).to_owned();

    // this method modifies theta
    // FIXME: Investigate if get_global_parallelism() messes with random numbers
    solve_upper_triangular_in_place(r.as_ref(), theta.as_mut(), get_global_parallelism());

    return (r_s.to_owned(), theta);
}

fn stop(maxit: usize, tol: f64, iter: usize, diff: f64) -> bool {
    iter >= maxit || diff < tol
}

pub fn distributed_binomial_single_iter_n(x: Mat<f64>, y: Mat<f64>, beta: Mat<f64>) -> Mat<f64> {
    let eta = x.clone() * beta;

    let mu = binomial::linkinv(&eta);
    let dmu = binomial::mu_eta(&eta);
    let z = eta + mat_div_elemwise(&(y - mu.clone()), &dmu.clone());
    let w = mat_div_elemwise(&mat_sqr_elemwise(&dmu.clone()), &binomial::variance(&mu));

    let sqrt_w = &mat_sqrt_elemwise(&w.clone());
    let x_tilde = &mul_elementwise(sqrt_w, &x);
    let z_tilde = &mul_elementwise(sqrt_w, &z);

    // r_local
    mat_cat(x_tilde, z_tilde, CatDim::HORIZONTAL)
        .qr()
        .R()
        .to_owned()
}

pub fn distributed_binomial_single_solve_n(
    r_local_with_all_r_remotes: Mat<f64>,
    beta: Mat<f64>,
    total_nrow: usize,
    maxit: usize,
    tol: f64,
    iter: usize,
) -> (Mat<f64>, Mat<f64>, bool) {
    let beta_old = beta.clone();

    let (r_local, beta) = ols_n(r_local_with_all_r_remotes);

    let vcov = vcov(r_local.clone(), FamilyEnum::BINOMIAL, total_nrow);
    let delta = mat_div_elemwise(
        &(beta_old - beta.clone()),
        &mat_sqrt_elemwise(&mat_diag(&vcov)),
    );
    let diff = delta.max().unwrap().abs();
    let stop = stop(maxit, tol, iter, diff);

    (r_local, beta, stop)
}
