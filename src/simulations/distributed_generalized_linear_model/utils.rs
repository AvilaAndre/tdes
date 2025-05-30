use faer::Mat;

pub fn div_elemwise(a: &Mat<f64>, b: &Mat<f64>) -> Mat<f64> {
    assert_eq!(a.nrows(), b.nrows(), "Mismatched number of rows");
    assert_eq!(a.ncols(), b.ncols(), "Mismatched number of columns");

    Mat::from_fn(a.nrows(), a.ncols(), |i, j| a.get(i, j) / b.get(i, j))
}

pub fn sqrt_elemwise(a: &Mat<f64>) -> Mat<f64> {
    Mat::from_fn(a.nrows(), a.ncols(), |i, j| a.get(i, j).sqrt())
}

pub fn sqr_elemwise(a: &Mat<f64>) -> Mat<f64> {
    Mat::from_fn(a.nrows(), a.ncols(), |i, j| a.get(i, j) * a.get(i, j))
}

/*
 * This method also broadcasts if possible
 */
pub fn mul_elementwise(a: &Mat<f64>, b: &Mat<f64>) -> Mat<f64> {
    let (nrows, ncols) = (a.nrows(), a.ncols());
    let (brows, bcols) = (b.nrows(), b.ncols());

    match (nrows, ncols, brows, bcols) {
        // Direct element-wise multiplication
        (ar, ac, br, bc) if ar == br && ac == bc => {
            Mat::from_fn(ar, ac, |i, j| a.get(i, j) * b.get(i, j))
        }
        // Column broadcast on b
        (ar, ac, br, 1) if ar == br => Mat::from_fn(ar, ac, |i, j| a.get(i, j) * b.get(i, 0)),
        // Column broadcast on a
        (ar, 1, br, bc) if ar == br => Mat::from_fn(br, bc, |i, j| a.get(i, 0) * b.get(i, j)),
        // Row broadcast on b
        (ar, ac, 1, bc) if ac == bc => Mat::from_fn(ar, ac, |i, j| a.get(i, j) * b.get(0, j)),
        // Row broadcast on a
        (1, ac, br, bc) if ac == bc => Mat::from_fn(br, bc, |i, j| a.get(0, j) * b.get(i, j)),
        // Scalar broadcast on b
        (ar, ac, 1, 1) => {
            let val = b.get(0, 0);
            Mat::from_fn(ar, ac, |i, j| a.get(i, j) * val)
        }
        // Scalar broadcast on a
        (1, 1, br, bc) => {
            let val = a.get(0, 0);
            Mat::from_fn(br, bc, |i, j| a.get(i, j) * val)
        }
        // Incompatible shapes
        _ => panic!("Cannot multiply matrices element-wise due to incompatible shapes"),
    }
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
