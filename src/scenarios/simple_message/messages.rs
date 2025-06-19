use crate::internal::core::Message;

#[derive(Debug, Clone)]
pub struct EmptyMessage {
    pub size: u64,
}

impl Message for EmptyMessage {
    fn size_bytes(&self) -> u64 {
        self.size
    }
}
