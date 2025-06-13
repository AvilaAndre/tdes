use std::collections::HashMap;

use faer::Mat;

use crate::internal::core::{Context, engine, log, macros::get_peer_of_type};

use super::{
    generalized_linear_model,
    message::{GlmConcatMessage, GlmSumRowsMessage},
    peer::GlmPeer,
    utils::{CatDim, mat_cat_vec},
};

pub fn peer_start(ctx: &mut Context, peer_id: usize) {
    broadcast_sum_rows(ctx, peer_id);
}

fn broadcast_sum_rows(ctx: &mut Context, peer_id: usize) {
    let mut nodes_filtered: Vec<usize> = Vec::new();
    if let Some(neighbors) = engine::get_neighbors_alive(ctx, peer_id) {
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

    let trace = match engine::send_message_to(ctx, peer_id, target_id, msg) {
        Some(lat) => format!("Sent GlmSumRowsMessage from {peer_id} to {target_id} in {lat}"),
        None => format!("Failed to send GlmConcatMessage from {peer_id} to {target_id}"),
    };
    log::trace(ctx, trace);
}

fn send_concat_r(ctx: &mut Context, peer_id: usize, target_id: usize) {
    let peer: &mut GlmPeer = get_peer_of_type!(ctx, peer_id, GlmPeer).expect("peer should exist");

    let msg = GlmConcatMessage {
        origin: peer_id,
        r_remote: peer.state.model.r_local.clone(),
        iter: peer.state.model.iter,
    };

    let trace = match engine::send_message_to(ctx, peer_id, target_id, msg) {
        Some(lat) => format!("Sent GlmSumRowsMessage from {peer_id} to {target_id} in {lat}"),
        None => format!("Failed to send GlmConcatMessage from {peer_id} to {target_id}"),
    };
    log::trace(ctx, trace);
}

pub fn receive_sum_rows_msg(ctx: &mut Context, peer_id: usize, msg: GlmSumRowsMessage) {
    let peer: &mut GlmPeer = get_peer_of_type!(ctx, peer_id, GlmPeer).expect("peer should exist");

    if !peer.state.r_n_rows.contains_key(&msg.origin) {
        // INFO: replaced r_remotes with r_n_rows
        peer.state.r_n_rows.insert(msg.origin, msg.nrows);

        if peer.state.nodes.len() == peer.state.r_n_rows.keys().len() {
            peer.state.total_nrow += peer.state.r_n_rows.values().sum::<usize>();
            peer.state.r_remotes = HashMap::new();
            // FIXME: Should r_remotes be reset too?
            broadcast_nodes(ctx, peer_id);
        }
    } else {
        log::warn(
            ctx,
            format!(
                "peer {peer_id} received sum_rows_msg but r_remotes already contained {}",
                msg.origin
            ),
        );
    }
}

pub fn receive_concat_r_msg(ctx: &mut Context, peer_id: usize, msg: GlmConcatMessage) {
    let peer: &mut GlmPeer = get_peer_of_type!(ctx, peer_id, GlmPeer).expect("peer should exist");

    // if does not have key msg.iter then insert HashMap::new()
    peer.state.r_remotes.entry(msg.iter).or_default();

    handle_iter(ctx, peer_id, msg.origin, msg.r_remote, msg.iter);
}

fn handle_iter(ctx: &mut Context, peer_id: usize, sender: usize, r_remote: Mat<f64>, iter: usize) {
    let peer: &mut GlmPeer = get_peer_of_type!(ctx, peer_id, GlmPeer).expect("peer should exist");

    if peer.state.r_remotes.contains_key(&iter)
        && !peer
            .state
            .r_remotes
            .get(&iter)
            .unwrap()
            .contains_key(&sender)
    {
        peer.state
            .r_remotes
            .get_mut(&iter)
            .and_then(|r| r.insert(sender, r_remote));

        if iter == peer.state.model.iter
            && peer.state.nodes.len() == peer.state.r_remotes.get(&iter).unwrap().keys().len()
        {
            let mut all_r_remotes = peer
                .state
                .r_remotes
                .get(&iter)
                .unwrap()
                .values()
                .cloned()
                .collect::<Vec<Mat<f64>>>();
            all_r_remotes.push(peer.state.model.r_local.clone());

            let r_local_with_all_r_remotes = mat_cat_vec(&all_r_remotes, CatDim::Vertical);

            let (r_local, beta, stop) = generalized_linear_model::distributed_single_solve_n(
                &r_local_with_all_r_remotes,
                &peer.state.model.coefficients,
                peer.state.model.family,
                peer.state.total_nrow,
                generalized_linear_model::DEFAULT_MAXIT,
                generalized_linear_model::DEFAULT_TOL,
                peer.state.model.iter,
            );

            peer.state.model.r_local = r_local;
            peer.state.model.coefficients = beta.clone();
            peer.state.model.iter += 1;
            peer.state.finished = stop;

            // INFO: Did not add branch where simulation stops
            if !stop {
                peer.state.model.r_local = generalized_linear_model::distributed_single_iter_n(
                    peer.state.model.family,
                    &peer.state.data.x,
                    &peer.state.data.y,
                    beta,
                );

                broadcast_nodes(ctx, peer_id);
            }
        } else {
            let first_check = !peer.state.r_remotes.contains_key(&iter);
            if first_check {
                log::warn(
                    ctx,
                    format!(
                        "on peer {peer_id} handle_iter failed because iter {iter} not in r_remotes"
                    ),
                );
            } else {
                log::warn(
                    ctx,
                    format!(
                        "on peer {peer_id} handle_iter failed because r_remotes[{iter}] does not contain sender {sender}"
                    ),
                );
            }
        }
    }
}
