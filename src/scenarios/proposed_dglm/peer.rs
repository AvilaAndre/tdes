use std::{
    collections::{BTreeSet, HashMap, hash_map},
    hash::{Hash, Hasher},
};

use faer::Mat;

use crate::internal::core::{
    macros::define_custom_peer,
    peer::{CustomPeer, PeerInfo},
};

use super::{
    ModelData, callbacks,
    family::FamilyEnum,
    generalized_linear_model::{self, GeneralizedLinearModel},
};

pub struct PGlmState {
    pub initial_model: GeneralizedLinearModel,
    pub model: GeneralizedLinearModel,
    pub data: ModelData,
    pub r_n_rows: HashMap<usize, usize>, // how many rows remotes have
    pub r_matrices: HashMap<usize, HashMap<usize, Mat<f64>>>,
    pub local_nrow: usize,
    pub total_nrow: usize,
    pub nodes: BTreeSet<usize>,
    pub neighbors: Vec<usize>,
    pub finished: bool,
    pub hash: u64,
}

pub struct PGlmPeer {
    pub peer_info: PeerInfo,
    pub state: PGlmState,
}

define_custom_peer!(PGlmPeer);

impl PGlmPeer {
    pub fn new(pos_x: f64, pos_y: f64, x: Mat<f64>, y: Mat<f64>) -> Self {
        let (r, c) = x.shape();
        let beta: Mat<f64> = Mat::zeros(c, 1);

        // INFO: Gaussian does not work
        let family: FamilyEnum = FamilyEnum::Binomial;

        let r_local =
            generalized_linear_model::distributed_single_iter_n(family, &x, &y, beta.clone());

        let initial_model = GeneralizedLinearModel {
            r_local,
            coefficients: beta.clone(),
            family,
            iter: 0,
        };

        Self {
            peer_info: PeerInfo::new(pos_x, pos_y, 0.0)
                .with_on_message_receive(callbacks::on_message_receive),
            state: PGlmState {
                model: initial_model.clone(),
                initial_model,
                data: ModelData { x, y },
                r_n_rows: HashMap::new(),
                r_matrices: HashMap::new(),
                local_nrow: r,
                total_nrow: r,
                nodes: BTreeSet::new(),
                neighbors: Vec::new(),
                finished: false,
                hash: 0,
            },
        }
    }

    pub fn update_hash(&mut self) {
        let mut s = hash_map::DefaultHasher::new();
        self.state.nodes.hash(&mut s);
        self.state.hash = s.finish()
    }

    pub fn discovery_reset(&mut self) {
        self.state.r_n_rows = HashMap::new();
        self.state.r_matrices = HashMap::new();
        self.state.total_nrow = 0;
        self.state.finished = false;
        self.state.model = self.state.initial_model.clone();

        let peer_id = self.get_id();
        self.state
            .r_matrices
            .entry(0)
            .or_default()
            .insert(peer_id, self.state.model.r_local.clone());

        self.update_hash();
    }
}
