use crate::{
    internal::core::{
        Context, engine, events::Timer, log, macros::get_peer_of_type, peer::CustomPeer,
    },
    scenarios::proposed_dglm::{algorithms, peer::PGlmPeer},
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
        let peer: &mut PGlmPeer =
            get_peer_of_type!(ctx, self.peer_id, PGlmPeer).expect("peer should exist");
        if !peer.is_alive() {
            return;
        }
        algorithms::get_node_ids(ctx, self.peer_id);
        algorithms::broadcast_sum_rows(ctx, self.peer_id);
    }
}

#[derive(Debug, Clone)]
pub struct TickTimer {
    pub interval: f64,
}

impl Timer for TickTimer {
    fn fire(&self, ctx: &mut Context) {
        for peer_id in 0..ctx.peers.len() {
            if ctx.peers[peer_id].is::<PGlmPeer>() {
                let peer: &mut PGlmPeer =
                    get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");
                if !peer.is_alive() {
                    continue;
                }
                algorithms::tick(ctx, peer_id);
            }
        }

        let all_peers_finished: bool = ctx
            .peers
            .iter()
            .filter_map(|p| p.downcast_ref::<PGlmPeer>())
            .all(|p| p.state.finished);

        if !all_peers_finished {
            engine::add_timer(ctx, ctx.clock + self.interval, self.clone());
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReviveTimer {
    pub target: usize,
}

impl Timer for ReviveTimer {
    fn fire(&self, ctx: &mut Context) {
        if let Some(target) = ctx.peers.get_mut(self.target) {
            target.revive();
            log::info(ctx, format!("Peer {} revived.", self.target));
            algorithms::get_node_ids(ctx, self.target);
            algorithms::broadcast_sum_rows(ctx, self.target);
        }
    }
}
