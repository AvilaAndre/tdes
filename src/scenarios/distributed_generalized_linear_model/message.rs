use faer::Mat;

use crate::internal::core::Message;

#[derive(Debug, Clone, Copy)]
pub struct GlmSumRowsMessage {
    pub origin: usize,
    pub nrows: usize,
}
impl Message for GlmSumRowsMessage {
    fn size_bytes(&self) -> u64 {
        1 + 1
    }
}

#[derive(Debug, Clone)]
pub struct GlmConcatMessage {
    pub origin: usize,
    pub r_remote: Mat<f64>,
    pub iter: usize,
}
impl Message for GlmConcatMessage {
    fn size_bytes(&self) -> u64 {
        let (r, c) = self.r_remote.shape();

        (1 + r * c * 8 + 1) as u64
    }
}
