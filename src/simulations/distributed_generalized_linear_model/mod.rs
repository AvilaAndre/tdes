mod algorithms;
mod callbacks;
mod hooks;
mod message;
mod peer;
mod timer;

mod family;
mod generalized_linear_model;
mod utils;

use algorithms::peer_start;
use faer::Mat;
use ordered_float::OrderedFloat;
use peer::GlmPeer;
use rand::{Rng, distr::Uniform};

use crate::internal::context::Context;

use csv::ReaderBuilder;
use std::error::Error;
use std::path::Path;

#[derive(Debug)]
struct ModelData {
    x: Mat<f64>,
    y: Mat<f64>,
}

fn model_data(m: &str) -> Result<ModelData, Box<dyn Error>> {
    let path = format!("./resources/generalized_linear_model/{}_mm.csv", m);
    let path = Path::new(&path).canonicalize()?;

    let mut x_data: Vec<Vec<f64>> = Vec::new();
    let mut y_data: Vec<Vec<f64>> = Vec::new();

    let mut reader = ReaderBuilder::new().has_headers(true).from_path(path)?;

    for result in reader.records() {
        let record = result?;
        let float_row: Vec<f64> = record
            .iter()
            .map(|s| s.parse::<f64>())
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

fn model_beta(m: &str) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let path = format!("./resources/generalized_linear_model/{}_beta.csv", m);
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

fn chunk_nx(mat: Mat<f64>, n: usize) -> Vec<Mat<f64>> {
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

pub fn start(ctx: &mut Context) {
    ctx.on_simulation_finish_hook = Some(hooks::on_simulation_finish_hook);

    let data: ModelData = match model_data("glm") {
        Ok(d) => d,
        Err(e) => panic!("Failed to load model_data: {}", e),
    };

    let _beta = match model_beta("glm") {
        Ok(b) => b,
        Err(e) => panic!("Failed to load model_beta: {}", e),
    };

    println!("_beta {:?}", _beta);

    let n_peers: usize = 7;

    let y_len = data.y.nrows();
    let ncols = data.x.ncols();

    assert!(data.x.nrows() == y_len, "x.nrows() != y.nrows()");
    assert!(n_peers * (ncols + 1) < y_len, "split > ncols");

    let x_chunks = chunk_nx(data.x, n_peers);
    let y_chunks = chunk_nx(data.y, n_peers);

    for (x, y) in x_chunks.into_iter().zip(y_chunks.into_iter()) {
        let (rx, ry) = (
            ctx.rng.sample(Uniform::new(-100.0, 100.0).unwrap()),
            ctx.rng.sample(Uniform::new(-100.0, 100.0).unwrap()),
        );

        let _ = ctx.add_peer(Box::new(GlmPeer::new(rx, ry, x, y)));
    }

    // full connection
    for i in 0..n_peers {
        for j in i + 1..n_peers {
            ctx.add_twoway_link(i, j, None);
        }
    }

    for i in 0..n_peers {
        peer_start(ctx, i);
    }

    ctx.run_for(OrderedFloat(17.1));
}
