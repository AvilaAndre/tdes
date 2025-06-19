use crate::{
    internal::core::{Context, engine, events::Timer, log},
    scenarios::proposed_dglm::{
        algorithms,
        improve::{self, tick},
        peer::PGlmPeer,
    },
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
        improve::get_node_ids(ctx, self.peer_id);
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
                tick(ctx, peer_id);
            }
        }

        let all_peers_finished: bool = ctx
            .peers
            .iter()
            .filter_map(|p| p.downcast_ref::<PGlmPeer>())
            .all(|p| p.state.finished);

        // println!("All finished? {all_peers_finished}");

        if !all_peers_finished {
            engine::add_timer(ctx, ctx.clock + self.interval, self.clone());
        }

        /*
         *
        let all_peers_finished: bool = ctx
            .peers
            .iter()
            .filter_map(|p| p.downcast_ref::<PGlmPeer>())
            .all(|p| p.state.r_n_rows.len() == p.state.nodes.len() - 1);
         *
         *
        for peer_id in 0..ctx.peers.len() {
            if ctx.peers[peer_id].is::<PGlmPeer>() {
                discovery_tick(ctx, peer_id);
            }
        }

        // check if still needed
        let all_peers_finished: bool = ctx
            .peers
            .iter()
            .filter_map(|p| p.downcast_ref::<PGlmPeer>())
            .all(|p| p.discovery.finished);

        if !all_peers_finished {
            engine::add_timer(ctx, ctx.clock + self.interval, self.clone());
        }
        */
    }
}
