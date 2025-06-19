use crate::{
    internal::core::{Context, events::Timer, log},
    scenarios::distributed_generalized_linear_model::algorithms,
};

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

#[derive(Debug, Clone)]
pub struct StartTimer {
    pub peer_id: usize,
}

impl Timer for StartTimer {
    fn fire(&self, ctx: &mut Context) {
        algorithms::broadcast_sum_rows(ctx, self.peer_id);
    }
}
