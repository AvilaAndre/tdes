use crate::{
    internal::core::{Context, engine, events::Timer, log},
    scenarios::simple_message::messages::EmptyMessage,
};

#[derive(Debug, Clone)]
pub struct StartTimer {
    pub message_size: u64,
}

impl Timer for StartTimer {
    fn fire(&self, ctx: &mut Context) {
        if let Some(neighbors) = engine::get_neighbors_alive(ctx, 0) {
            for neigh_id in neighbors {
                log::info(ctx, format!("Peer 0 sent message to {neigh_id}"));
                engine::send_message_to(
                    ctx,
                    0,
                    neigh_id,
                    EmptyMessage {
                        size: self.message_size,
                    },
                );
            }
        }
    }
}
