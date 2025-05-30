use faer::Mat;

pub fn linkinv(eta: &Mat<f64>) -> Mat<f64> {
    eta.clone()
}

pub fn mu_eta(eta: &Mat<f64>) -> Mat<f64> {
    Mat::ones(eta.nrows(), eta.ncols())
}

pub fn variance(mu: &Mat<f64>) -> Mat<f64> {
    Mat::ones(mu.nrows(), mu.ncols())
}
