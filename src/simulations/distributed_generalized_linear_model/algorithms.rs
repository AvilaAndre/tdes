use std::collections::HashMap;

use crate::{
    get_peer_of_type,
    internal::{context::Context, message_passing::send_message_to},
};

use super::{message::GlmSumRowsMessage, peer::GlmPeer};

pub fn peer_start(ctx: &mut Context, peer_id: usize) {
    broadcast_sum_rows(ctx, peer_id)
}

fn broadcast_sum_rows(ctx: &mut Context, peer_id: usize) {
    let mut nodes_filtered: Vec<usize> = Vec::new();
    if let Some(neighbors) = ctx.get_neighbors(peer_id) {
        for neigh_id in neighbors {
            if ctx.peers.get(neigh_id).is_some_and(|p| p.is::<GlmPeer>()) {
                send_sum_rows(ctx, peer_id, neigh_id);
                nodes_filtered.push(neigh_id);
            }
        }
    }

    let peer: &mut GlmPeer = get_peer_of_type!(ctx, peer_id, GlmPeer).expect("peer should exist");

    peer.state.nodes = nodes_filtered;
}

fn broadcast_nodes(ctx: &mut Context, peer_id: usize) {
    let peer: &mut GlmPeer = get_peer_of_type!(ctx, peer_id, GlmPeer).expect("peer should exist");

    let neigh_ids = peer.state.nodes.clone();
    for neigh_id in neigh_ids {
        send_concat_r(ctx, peer_id, neigh_id);
    }
}

fn send_sum_rows(ctx: &mut Context, peer_id: usize, target_id: usize) {
    let peer: &mut GlmPeer = get_peer_of_type!(ctx, peer_id, GlmPeer).expect("peer should exist");

    let msg = GlmSumRowsMessage {
        origin: peer_id,
        nrows: peer.state.total_nrow,
    };

    send_message_to(ctx, peer_id, target_id, Some(Box::new(msg)));
}

fn send_concat_r(ctx: &mut Context, peer_id: usize, target_id: usize) {
    todo!()
}

pub fn receive_sum_rows_msg(ctx: &mut Context, peer_id: usize, msg: GlmSumRowsMessage) {
    let peer: &mut GlmPeer = get_peer_of_type!(ctx, peer_id, GlmPeer).expect("peer should exist");

    // sender, nrows = msg.origin, msg.nrows
    if peer.state.r_remotes.contains_key(&msg.origin) {
        peer.state.r_remotes.insert(msg.origin, msg.nrows);

        // TODO: nodes is not set on start
        if peer.state.nodes.len() == peer.state.r_remotes.keys().len() {
            peer.state.total_nrow += peer.state.r_remotes.keys().into_iter().sum::<usize>();
            peer.state.r_remotes = HashMap::new();
            broadcast_nodes(ctx, peer_id);
        }
    }
}

/*
   def receive_sum_rows_msg(self, msg: GLMSumRowsMessage):
       sender, nrows = msg.origin, msg.nrows

       if sender not in self.state.r_remotes.keys():
           self.state.r_remotes[sender] = nrows

           if len(self.state.nodes) == len(self.state.r_remotes.keys()):
               self.state.total_nrow += sum(self.state.r_remotes.values())
               self.broadcast_nodes()
               self.state.r_remotes = {}
*/
