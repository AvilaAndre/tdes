use crate::internal::core::Message;

#[derive(Debug, Clone)]
pub struct FlowUpdatingPairwiseMessage {
    pub sender: usize,
    pub flow: f64,
    pub estimate: f64,
}
impl Message for FlowUpdatingPairwiseMessage {
    fn size_bytes(&self) -> u64 {
        1 + 8 + 8
    }
}
