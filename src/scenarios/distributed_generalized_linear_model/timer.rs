use crate::internal::core::{Context, events::Timer, log};

#[derive(Debug, Clone)]
pub struct KillTimer {
    pub target: usize,
}

impl KillTimer {
    pub fn new(target: usize) -> Self {
        Self { target }
    }
}

impl Timer for KillTimer {
    fn fire(&self, ctx: &mut Context) {
        if let Some(target) = ctx.peers.get_mut(self.target) {
            target.kill();
            log::info(ctx, format!("Peer {} killed.", self.target));
        }
    }
}
