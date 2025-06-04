use csv::ReaderBuilder;
use faer::Mat;
use std::error::Error;
use std::path::Path;

#[derive(Debug)]
pub struct ModelData {
    pub x: Mat<f64>,
    pub y: Mat<f64>,
}

pub fn model_data(m: &str) -> Result<ModelData, Box<dyn Error>> {
    let path = format!("./resources/generalized_linear_model/{m}_mm.csv");
    let path = Path::new(&path).canonicalize()?;

    let mut x_data: Vec<Vec<f64>> = Vec::new();
    let mut y_data: Vec<Vec<f64>> = Vec::new();

    let mut reader = ReaderBuilder::new().has_headers(true).from_path(path)?;

    for result in reader.records() {
        let record = result?;
        let float_row: Vec<f64> = record
            .iter()
            .map(str::parse::<f64>)
            .collect::<Result<Vec<f64>, _>>()?;

        if !float_row.is_empty() {
            let (y, x) = float_row.split_last().unwrap();

            x_data.push(x.to_vec());
            y_data.push(vec![*y]);
        }
    }

    Ok(ModelData {
        x: Mat::from_fn(x_data.len(), x_data[0].len(), |i, j| x_data[i][j]),
        y: Mat::from_fn(y_data.len(), y_data[0].len(), |i, j| y_data[i][j]),
    })
}

pub fn model_beta(m: &str) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let path = format!("./resources/generalized_linear_model/{m}_beta.csv");
    let path = Path::new(&path).canonicalize()?;

    let mut result: Vec<Vec<f64>> = Vec::new();

    let mut reader = ReaderBuilder::new().has_headers(true).from_path(path)?;

    for record_result in reader.records() {
        let record = record_result?;
        if let Some(first_field) = record.get(0) {
            let value: f64 = first_field.parse()?;
            result.push(vec![value]);
        }
    }

    Ok(result)
}

pub fn chunk_nx(mat: Mat<f64>, n: usize) -> Vec<Mat<f64>> {
    if n == 1 {
        return vec![mat];
    }

    let nsplits = mat.nrows() / n;
    let mut chunks = Vec::new();
    let mut start_row = 0;

    // Create n-1 chunks of equal size
    for _ in 0..(n - 1) {
        let chunk = mat.subrows(start_row, nsplits);
        chunks.push(chunk.to_owned());
        start_row += nsplits;
    }

    // Add the remaining rows as the last chunk
    let remaining_rows = mat.nrows() - start_row;
    let last_chunk = mat.subrows(start_row, remaining_rows);
    chunks.push(last_chunk.to_owned());

    chunks
}
