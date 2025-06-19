use faer::Mat;

use crate::{
    internal::core::{Context, engine, log, macros::get_peer_of_type},
    scenarios::proposed_dglm::{improve::check_missing_concat_r, message::PGlmSumRowsMessage},
};

use super::{
    generalized_linear_model,
    message::GlmConcatMessage,
    peer::PGlmPeer,
    utils::{CatDim, mat_cat_vec},
};

pub fn broadcast_sum_rows(ctx: &mut Context, peer_id: usize) {
    let peer: &mut PGlmPeer = get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");

    for neigh_id in peer.state.neighbors.clone() {
        send_sum_rows(ctx, peer_id, neigh_id);
    }
}

fn send_sum_rows(ctx: &mut Context, peer_id: usize, target_id: usize) {
    let peer: &mut PGlmPeer = get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");

    let msg = PGlmSumRowsMessage {
        origin: peer_id,
        nrows: peer.state.total_nrow,
        hash: peer.state.hash,
    };

    let trace = match engine::send_message_to(ctx, peer_id, target_id, msg) {
        Some(lat) => format!("Sent GlmSumRowsMessage from {peer_id} to {target_id} in {lat}"),
        None => format!("Failed to send GlmSumRowsMessage from {peer_id} to {target_id}"),
    };
    log::trace(ctx, trace);
}

pub fn receive_concat_r_msg(ctx: &mut Context, peer_id: usize, msg: GlmConcatMessage) {
    log::trace(
        ctx,
        format!(
            "peer {peer_id} received a GlmConcatMessage from {sender} on iteration {iter}",
            sender = msg.origin,
            iter = msg.iter
        ),
    );

    let peer: &mut PGlmPeer = get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");

    // if does not have key msg.iter then insert HashMap::new()
    peer.state.r_matrices.entry(msg.iter).or_default();

    handle_iter(ctx, peer_id, msg.origin, msg.r_remote, msg.iter);
}

fn handle_iter(ctx: &mut Context, peer_id: usize, sender: usize, r_remote: Mat<f64>, iter: usize) {
    let peer: &mut PGlmPeer = get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");

    if peer.state.r_matrices.contains_key(&iter)
        && !peer
            .state
            .r_matrices
            .get(&iter)
            .unwrap()
            .contains_key(&sender)
    {
        peer.state
            .r_matrices
            .get_mut(&iter)
            .and_then(|r| r.insert(sender, r_remote));

        let current_iter = peer.state.model.iter;
        let msgs_to_receive = peer.state.nodes.len();
        let msgs_received_in_iter = peer.state.r_matrices.get(&iter).unwrap().keys().len();

        if iter == current_iter && msgs_to_receive == msgs_received_in_iter {
            let all_r_remotes = peer
                .state
                .r_matrices
                .get(&iter)
                .unwrap()
                .values()
                .cloned()
                .collect::<Vec<Mat<f64>>>();

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

            peer.state.model.r_local = r_local.clone();
            peer.state.model.coefficients = beta.clone();
            peer.state.model.iter += 1;
            peer.state.finished = stop;

            let new_current_iter = peer.state.model.iter;
            // add r_local to r_matrices
            peer.state
                .r_matrices
                .entry(new_current_iter)
                .or_default()
                .insert(peer_id, r_local);

            if !stop {
                peer.state.model.r_local = generalized_linear_model::distributed_single_iter_n(
                    peer.state.model.family,
                    &peer.state.data.x,
                    &peer.state.data.y,
                    beta,
                );

                // add r_local to r_matrices
                peer.state
                    .r_matrices
                    .entry(new_current_iter)
                    .or_default()
                    .insert(peer_id, peer.state.model.r_local.clone());

                log::info(
                    ctx,
                    format!(
                        "peer {peer_id} broadcasting as it starts iteration {new_current_iter}"
                    ),
                );

                check_missing_concat_r(ctx, peer_id);
            } else {
                log::info(ctx, format!("peer {peer_id} finished on iteration {iter}"));
            }
        } else if msgs_to_receive == msgs_received_in_iter {
            log::warn(
                ctx,
                format!(
                    "peer {peer_id} handle_iter, received all messages from iteration {iter} but current iteration is {current_iter}",
                ),
            );
        } else {
            log::trace(
                ctx,
                format!(
                    "peer {peer_id} handle_iter, received ({msgs_received_in_iter}/{msgs_to_receive}) messages on iteration {iter} (current iteration is {current_iter})",
                ),
            );
        }
    } else {
        log::warn(
            ctx,
            format!(
                "peer {peer_id} handle_iter failed because r_remotes[{iter}] already contains sender {sender}"
            ),
        );
    }
}
