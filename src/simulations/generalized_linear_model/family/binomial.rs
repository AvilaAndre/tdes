use faer::Mat;

pub fn linkinv(eta: &Mat<f64>) -> Mat<f64> {
    Mat::from_fn(eta.nrows(), eta.ncols(), |i, j| {
        let x = eta.get(i, j);
        1.0 / (1.0 + (-x).exp())
    })
}

pub fn mu_eta(eta: &Mat<f64>) -> Mat<f64> {
    Mat::from_fn(eta.nrows(), eta.ncols(), |i, j| {
        let x = eta.get(i, j);
        let exp = x.exp();
        exp / ((1.0 + exp).powi(2))
    })
}

pub fn variance(mu: &Mat<f64>) -> Mat<f64> {
    Mat::from_fn(mu.nrows(), mu.ncols(), |i, j| {
        let x = mu.get(i, j);
        x * (1.0 - x)
    })
}

