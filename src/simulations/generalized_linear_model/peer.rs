use std::collections::HashMap;

use faer::Mat;

use crate::internal::peer::{CustomPeer, Peer};

use super::{
    ModelData,
    family::FamilyEnum,
    generalized_linear_model::{self, GeneralizedLinearModel},
};

pub struct GlmState {
    pub model: GeneralizedLinearModel,
    pub data: ModelData,
    pub r_remotes: HashMap<usize, i32>,
    pub total_nrow: usize,
    pub nodes: Vec<usize>,
    pub finished: bool,
}

pub struct GlmPeer {
    pub peer: Peer,
    pub state: GlmState,
}

impl GlmPeer {
    pub fn new(pos_x: f64, pos_y: f64, x: Mat<f64>, y: Mat<f64>) -> Self {
        let (r, c) = x.shape();
        let beta: Mat<f64> = Mat::zeros(c, 1);

        let r_local = generalized_linear_model::distributed_binomial_single_iter_n(
            x.clone(),
            y.clone(),
            beta.clone(),
        );

        let model = GeneralizedLinearModel {
            r_local,
            coefficients: beta,
            family: FamilyEnum::BINOMIAL,
            iter: 0,
        };

        Self {
            peer: {
                // TODO: implement message received callback
                // this.on_message_receive = callbacks::example_on_message_receive;
                Peer::new(pos_x, pos_y, 0.0)
            },

            state: GlmState {
                model: model,
                data: ModelData { x, y },
                r_remotes: HashMap::new(),
                total_nrow: r,
                nodes: Vec::new(),
                finished: false,
            },
        }
    }
}

impl CustomPeer for GlmPeer {
    fn get_peer(&self) -> &Peer {
        &self.peer
    }

    fn get_peer_mut(&mut self) -> &mut Peer {
        &mut self.peer
    }
}
