use crate::internal::core::Message;

#[derive(Debug, Clone)]
pub struct FlowUpdatingPairwiseMessage {
    pub sender: usize,
    pub flow: f64,
    pub estimate: f64,
}
impl Message for FlowUpdatingPairwiseMessage {}
