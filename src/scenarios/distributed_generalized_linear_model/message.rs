use faer::Mat;

use crate::internal::core::Message;

#[derive(Debug, Clone, Copy)]
pub struct GlmSumRowsMessage {
    pub origin: usize,
    pub nrows: usize,
}
impl Message for GlmSumRowsMessage {}

#[derive(Debug, Clone)]
pub struct GlmConcatMessage {
    pub origin: usize,
    pub r_remote: Mat<f64>,
    pub iter: usize,
}
impl Message for GlmConcatMessage {}
