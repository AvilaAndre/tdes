use std::collections::{BTreeSet, HashMap};

use faer::Mat;

use crate::{
    internal::core::{Context, engine, log, macros::get_peer_of_type, peer::CustomPeer},
    scenarios::proposed_dglm::{
        message::{GlmConcatMessage, PGlmSumRowsMessage, ReqConcatMessage, ReqSumRowsMessage},
        peer::PGlmPeer,
    },
};

pub fn get_node_ids(ctx: &mut Context, peer_id: usize) {
    log::trace(ctx, format!("peer {peer_id} get node ids"));

    let mut peer_ids_filtered: Vec<usize> = engine::get_neighbors_alive(ctx, peer_id)
        .map(|neighbors| {
            neighbors
                .into_iter()
                .filter(|&neigh_id| ctx.peers.get(neigh_id).is_some_and(|p| p.is::<PGlmPeer>()))
                .collect()
        })
        .unwrap_or_default();

    let peer: &mut PGlmPeer = get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");

    peer.state.neighbors = peer_ids_filtered.clone();

    peer_ids_filtered.push(peer.get_id());
    peer.state.nodes = BTreeSet::from_iter(peer_ids_filtered);
    peer.discovery_reset();
    log::trace(ctx, format!("peer {peer_id} called discovery_reset"));
}

pub fn broadcast_sum_rows_req(ctx: &mut Context, peer_id: usize, msg: ReqSumRowsMessage) {
    let peer: &mut PGlmPeer = get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");

    for neigh_id in peer.state.neighbors.clone() {
        let trace = match engine::send_message_to(ctx, peer_id, neigh_id, msg.clone()) {
            Some(lat) => format!("Sent ReqSumRowsMessage from {peer_id} to {neigh_id} in {lat}"),
            None => format!("Failed to send ReqSumRowsMessage from {peer_id} to {neigh_id}"),
        };
        log::trace(ctx, trace);
    }
}

pub fn check_missing_sum_rows(ctx: &mut Context, peer_id: usize) -> bool {
    let peer: &mut PGlmPeer = get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");

    let mut sum_rows_to_req: Vec<usize> = vec![];
    for neigh_id in peer.state.nodes.iter() {
        if !peer.state.r_n_rows.contains_key(neigh_id) && *neigh_id != peer_id {
            sum_rows_to_req.push(*neigh_id);
        }
    }

    let hash = peer.state.hash;
    let nodes = peer.state.nodes.clone();
    let neighbors = peer.state.neighbors.clone();

    log::debug(
        ctx,
        format!(
            "tick check check_missing_sum_rows peer {peer_id} misses: {sum_rows_to_req:?}, {nodes:?}, neighbors: {neighbors:?}"
        ),
    );

    if !sum_rows_to_req.is_empty() {
        log::trace(
            ctx,
            format!("peer {peer_id} has missing rows {sum_rows_to_req:?}"),
        );
        let msg = ReqSumRowsMessage {
            needs: sum_rows_to_req.clone(),
            hash,
        };
        broadcast_sum_rows_req(ctx, peer_id, msg);
        true
    } else {
        false
    }
}

pub fn receive_sum_rows_req_msg(
    ctx: &mut Context,
    peer_id: usize,
    requester_id: usize,
    msg: ReqSumRowsMessage,
) {
    let peer: &mut PGlmPeer = get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");

    let mut has: Vec<usize> = peer.state.r_n_rows.keys().copied().collect();
    has.push(peer_id);

    log::trace(
        ctx,
        format!(
            "peer {peer_id} received a request for rows from {rows:?}, it has: {has:?}",
            rows = msg.needs
        ),
    );

    for p_id in msg.needs {
        let peer: &mut PGlmPeer =
            get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");

        let mut n_rows: usize = 0;
        let hash = peer.state.hash;

        let mut has_rows = false;
        if let Some(n) = peer.state.r_n_rows.get(&p_id) {
            n_rows = *n;
            has_rows = true;
        } else if p_id == peer_id {
            n_rows = peer.state.local_nrow;
            has_rows = true;
        }

        if has_rows {
            engine::send_message_to(
                ctx,
                peer_id,
                requester_id,
                PGlmSumRowsMessage {
                    origin: p_id,
                    nrows: n_rows,
                    hash,
                },
            );
        }
    }
}

pub fn tick(ctx: &mut Context, peer_id: usize) {
    if check_missing_sum_rows(ctx, peer_id) {
        return;
    }
    check_missing_concat_r(ctx, peer_id);
}

pub fn receive_sum_rows_msg(ctx: &mut Context, peer_id: usize, msg: PGlmSumRowsMessage) {
    log::trace(
        ctx,
        format!(
            "peer {peer_id} received a PGlmSumRowsMessage message from peer {sender} with nrows: {nrows}",
            sender = msg.origin,
            nrows = msg.nrows
        ),
    );

    let peer: &mut PGlmPeer = get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");

    if let std::collections::hash_map::Entry::Vacant(e) = peer.state.r_n_rows.entry(msg.origin) {
        e.insert(msg.nrows);

        if peer.state.nodes.len() - 1 == peer.state.r_n_rows.keys().len() {
            peer.state.total_nrow =
                peer.state.local_nrow + peer.state.r_n_rows.values().sum::<usize>();
            peer.state.r_matrices = HashMap::new();
            // r_matrices should include initial r_local
            peer.state
                .r_matrices
                .entry(0)
                .or_default()
                .insert(peer_id, peer.state.model.r_local.clone());

            peer.state.model = peer.state.initial_model.clone();

            check_missing_concat_r(ctx, peer_id);
        }
    } else {
        log::warn(
            ctx,
            format!(
                "peer {peer_id} received GlmSumRowsMessage but r_remotes already contained {}",
                msg.origin
            ),
        );
    }
}

pub fn broadcast_concat_req(ctx: &mut Context, peer_id: usize, msg: ReqConcatMessage) {
    let peer: &mut PGlmPeer = get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");

    for neigh_id in peer.state.neighbors.clone() {
        let trace = match engine::send_message_to(ctx, peer_id, neigh_id, msg.clone()) {
            Some(lat) => format!("Sent ReqConcatMessage from {peer_id} to {neigh_id} in {lat}"),
            None => format!("Failed to send ReqConcatMessage from {peer_id} to {neigh_id}"),
        };
        log::trace(ctx, trace);
    }
}

pub fn receive_concat_r_req_msg(
    ctx: &mut Context,
    peer_id: usize,
    requester_id: usize,
    msg: ReqConcatMessage,
) {
    let peer: &mut PGlmPeer = get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");

    let peer_iter = peer.state.model.iter;
    let msg_iter = msg.iter;

    if msg.iter > peer_iter {
        log::trace(
            ctx,
            format!(
                "peer {peer_id} received a request for R local matrix in iter {msg_iter} but is on iter {peer_iter}",
            ),
        );
        return;
    }

    let has: Vec<usize> = peer
        .state
        .r_matrices
        .entry(msg_iter)
        .or_default()
        .keys()
        .copied()
        .collect();

    log::trace(
        ctx,
        format!(
            "peer {peer_id} received a request for R local matrix on iter {msg_iter} from {requester_id} for {rs:?}, it has: {has:?}",
            rs = msg.needs
        ),
    );

    for p_id in msg.needs {
        let peer: &mut PGlmPeer =
            get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");

        let mut r_to_send: Mat<f64> = Mat::new();
        let hash = peer.state.hash;

        let mut has_r = false;
        if let Some(r) = peer
            .state
            .r_matrices
            .entry(msg.iter)
            .or_default()
            .get(&p_id)
        {
            r_to_send = r.clone();
            has_r = true;
        } else if p_id == peer_id {
            panic!(
                "peer_id {peer_id} r_local must exist in r_matrices r_matrice msg_iter {msg_iter} r_matrices: {:?} r_matrices[{msg_iter}]: {:?}",
                peer.state.r_matrices.keys().copied().collect::<Vec<_>>(),
                peer.state
                    .r_matrices
                    .entry(msg_iter)
                    .or_default()
                    .keys()
                    .collect::<Vec<_>>(),
            );
        }

        if has_r {
            engine::send_message_to(
                ctx,
                peer_id,
                requester_id,
                GlmConcatMessage {
                    origin: p_id,
                    r_remote: r_to_send,
                    iter: msg.iter,
                    hash,
                },
            );
        }
    }
}

pub fn check_missing_concat_r(ctx: &mut Context, peer_id: usize) -> bool {
    let peer: &mut PGlmPeer = get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");

    let iter = peer.state.model.iter;

    let mut concat_r_to_req: Vec<usize> = vec![];
    for neigh_id in peer.state.nodes.iter() {
        if !peer
            .state
            .r_matrices
            .entry(iter)
            .or_default()
            .contains_key(neigh_id)
            && *neigh_id != peer_id
        {
            concat_r_to_req.push(*neigh_id);
        }
    }

    let hash = peer.state.hash;

    if !concat_r_to_req.is_empty() {
        log::trace(
            ctx,
            format!("peer {peer_id} has missing r_matrices[{iter}] {concat_r_to_req:?}"),
        );
        let msg = ReqConcatMessage {
            needs: concat_r_to_req.clone(),
            iter,
            hash,
        };
        broadcast_concat_req(ctx, peer_id, msg);
        true
    } else {
        false
    }
}
