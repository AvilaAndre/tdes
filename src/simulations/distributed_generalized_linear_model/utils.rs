use faer::Mat;

pub fn mat_div_elemwise(a: &Mat<f64>, b: &Mat<f64>) -> Mat<f64> {
    assert_eq!(a.nrows(), b.nrows(), "Mismatched number of rows");
    assert_eq!(a.ncols(), b.ncols(), "Mismatched number of columns");

    Mat::from_fn(a.nrows(), a.ncols(), |i, j| a.get(i, j) / b.get(i, j))
}

pub fn mat_sqrt_elemwise(a: &Mat<f64>) -> Mat<f64> {
    Mat::from_fn(a.nrows(), a.ncols(), |i, j| a.get(i, j).sqrt())
}

pub fn mat_sqr_elemwise(a: &Mat<f64>) -> Mat<f64> {
    Mat::from_fn(a.nrows(), a.ncols(), |i, j| a.get(i, j) * a.get(i, j))
}

pub fn mat_diag(a: &Mat<f64>) -> Mat<f64> {
    let sz = a.nrows().min(a.ncols());
    Mat::from_fn(sz, 1, |i, _| *a.get(i, i))
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

#[derive(Copy, Clone)]
pub enum CatDim {
    Vertical = 0,
    Horizontal = 1,
}
pub fn mat_cat(a: &Mat<f64>, b: &Mat<f64>, dim: CatDim) -> Mat<f64> {
    match dim {
        CatDim::Vertical => {
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
        CatDim::Horizontal => {
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

pub fn mat_cat_vec(mats: Vec<Mat<f64>>, dim: CatDim) -> Mat<f64> {
    assert!(
        !mats.is_empty(),
        "mat_cat_vec must receive at least one matrix"
    );

    let mut iterator = mats.iter();
    let mut result = iterator.next().unwrap().clone();

    for mat in iterator {
        result = mat_cat(&result, mat, dim);
    }

    result
}

const DEFAULT_RTOL: f64 = 1e-05;
const DEFAULT_ATOL: f64 = 1e-08;

pub fn mat_allclose(a: &Mat<f64>, b: &Mat<f64>, rtol: f64, atol: f64) -> bool {
    if a.nrows() != b.nrows() || a.ncols() != b.ncols() {
        return false;
    }

    for i in 0..a.nrows() {
        for j in 0..a.ncols() {
            let diff = (a.get(i, j) - b.get(i, j)).abs();
            let tolerance = atol + rtol * b.get(i, j).abs();
            if diff > tolerance {
                return false;
            }
        }
    }
    true
}

pub fn mat_allclose_default(a: &Mat<f64>, b: &Mat<f64>) -> bool {
    mat_allclose(a, b, DEFAULT_RTOL, DEFAULT_ATOL)
}
