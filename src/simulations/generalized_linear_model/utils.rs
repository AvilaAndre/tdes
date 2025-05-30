use faer::Mat;

pub fn div_elemwise(a: &Mat<f64>, b: &Mat<f64>) -> Mat<f64> {
    assert_eq!(a.nrows(), b.nrows(), "Mismatched number of rows");
    assert_eq!(a.ncols(), b.ncols(), "Mismatched number of columns");

    Mat::from_fn(a.nrows(), a.ncols(), |i, j| a.get(i, j) / b.get(i, j))
}

pub fn sqrt_elemwise(a: &Mat<f64>) -> Mat<f64> {
    Mat::from_fn(a.nrows(), a.ncols(), |i, j| a.get(i, j).sqrt())
}

pub enum CatDim {
    VERTICAL = 0,
    HORIZONTAL = 1,
}
pub fn mat_cat(a: &Mat<f64>, b: &Mat<f64>, dim: CatDim) -> Mat<f64> {
    match dim {
        CatDim::VERTICAL => {
            let ncols = a.ncols();
            let nrows_total = a.nrows() + b.nrows();

            assert_eq!(
                a.ncols(),
                b.ncols(),
                "Column count must match for vertical concatenation"
            );

            let mut out = Mat::<f64>::zeros(nrows_total, ncols);
            out.submatrix_mut(0, 0, a.nrows(), ncols).copy_from(a);
            out.submatrix_mut(a.nrows(), 0, b.nrows(), ncols)
                .copy_from(b);
            out
        }
        CatDim::HORIZONTAL => {
            let nrows = a.nrows();
            let ncols_total = a.ncols() + b.ncols();

            assert_eq!(
                a.nrows(),
                b.nrows(),
                "Row count must match for horizontal concatenation"
            );

            let mut out = Mat::<f64>::zeros(nrows, ncols_total);
            out.submatrix_mut(0, 0, nrows, a.ncols()).copy_from(a);
            out.submatrix_mut(0, a.ncols(), nrows, b.ncols())
                .copy_from(b);
            out
        }
    }
}
