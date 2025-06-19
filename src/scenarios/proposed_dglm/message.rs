use faer::Mat;

use crate::internal::core::Message;

#[derive(Debug, Clone)]
pub struct PGlmSumRowsMessage {
    pub origin: usize,
    pub nrows: usize,
    pub hash: u64,
}
impl Message for PGlmSumRowsMessage {
    fn size_bytes(&self) -> u64 {
        1 + 1 + 8
    }
}

#[derive(Debug, Clone)]
pub struct GlmConcatMessage {
    pub origin: usize,
    pub r_remote: Mat<f64>,
    pub iter: usize,
    pub hash: u64,
}
impl Message for GlmConcatMessage {
    fn size_bytes(&self) -> u64 {
        let (r, c) = self.r_remote.shape();

        (1 + r * c * 8 + 1 + 8) as u64
    }
}

#[derive(Debug, Clone)]
pub struct DiscoveryMessage {
    pub origin: usize,
    pub nodes: Vec<usize>,
}
impl Message for DiscoveryMessage {
    fn size_bytes(&self) -> u64 {
        let n_nodes = self.nodes.len();

        (1 + n_nodes) as u64
    }
}

#[derive(Debug, Clone)]
pub struct ReqSumRowsMessage {
    pub needs: Vec<usize>,
    pub hash: u64,
}
impl Message for ReqSumRowsMessage {
    fn size_bytes(&self) -> u64 {
        self.needs.len() as u64 + 8
    }
}

#[derive(Debug, Clone)]
pub struct ReqConcatMessage {
    pub needs: Vec<usize>,
    pub iter: usize,
    pub hash: u64,
}
impl Message for ReqConcatMessage {
    fn size_bytes(&self) -> u64 {
        self.needs.len() as u64 + 1 + 8
    }
}
