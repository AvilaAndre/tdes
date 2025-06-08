use std::collections::HashMap;

use faer::Mat;

use crate::internal::core::{
    macros::define_custom_peer,
    peer::{CustomPeer, Peer},
};

use super::{
    ModelData, callbacks,
    family::FamilyEnum,
    generalized_linear_model::{self, GeneralizedLinearModel},
};

pub struct GlmState {
    pub model: GeneralizedLinearModel,
    pub data: ModelData,
    pub r_n_rows: HashMap<usize, usize>, // how many rows remotes have
    pub r_remotes: HashMap<usize, HashMap<usize, Mat<f64>>>,
    pub total_nrow: usize,
    pub nodes: Vec<usize>,
    pub finished: bool,
}

pub struct GlmPeer {
    pub peer: Peer,
    pub state: GlmState,
}

define_custom_peer!(GlmPeer);

impl GlmPeer {
    pub fn new(pos_x: f64, pos_y: f64, x: Mat<f64>, y: Mat<f64>) -> Self {
        let (r, c) = x.shape();
        let beta: Mat<f64> = Mat::zeros(c, 1);

        // INFO: Gaussian does not work
        let family: FamilyEnum = FamilyEnum::Binomial;

        let r_local =
            generalized_linear_model::distributed_single_iter_n(family, &x, &y, beta.clone());

        let model = GeneralizedLinearModel {
            r_local,
            coefficients: beta.clone(),
            family,
            iter: 0,
        };

        Self {
            peer: Peer::new(pos_x, pos_y, 0.0)
                .with_on_message_receive(callbacks::on_message_receive),
            state: GlmState {
                model,
                data: ModelData { x, y },
                r_n_rows: HashMap::new(),
                r_remotes: HashMap::new(),
                total_nrow: r,
                nodes: Vec::new(),
                finished: false,
            },
        }
    }
}
