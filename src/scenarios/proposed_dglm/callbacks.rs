use crate::{
    internal::core::{Context, Message, log, macros::get_peer_of_type},
    scenarios::proposed_dglm::{
        algorithms::{receive_concat_r_req_msg, receive_sum_rows_msg, receive_sum_rows_req_msg},
        discovery::{receive_discovery_msg, send_discovery_msg},
        message::{DiscoveryMessage, PGlmSumRowsMessage, ReqConcatMessage, ReqSumRowsMessage},
        peer::PGlmPeer,
    },
};

use super::{algorithms::receive_concat_r_msg, message::GlmConcatMessage};

pub fn on_message_receive(
    ctx: &mut Context,
    sender_id: usize,
    receiver_id: usize,
    msg: &dyn Message,
) {
    if let Some(sum_rows_msg) = msg.downcast_ref::<PGlmSumRowsMessage>() {
        if operation_msg_hash_guard(ctx, sender_id, receiver_id, sum_rows_msg.hash) {
            receive_sum_rows_msg(ctx, receiver_id, sum_rows_msg.clone());
        }
    } else if let Some(concat_msg) = msg.downcast_ref::<GlmConcatMessage>() {
        if operation_msg_hash_guard(ctx, sender_id, receiver_id, concat_msg.hash) {
            receive_concat_r_msg(ctx, receiver_id, concat_msg.clone());
        }
    } else if let Some(discovery_req_msg) = msg.downcast_ref::<ReqSumRowsMessage>() {
        if operation_msg_hash_guard(ctx, sender_id, receiver_id, discovery_req_msg.hash) {
            receive_sum_rows_req_msg(ctx, receiver_id, sender_id, discovery_req_msg.clone());
        }
    } else if let Some(concat_req_msg) = msg.downcast_ref::<ReqConcatMessage>() {
        if operation_msg_hash_guard(ctx, sender_id, receiver_id, concat_req_msg.hash) {
            receive_concat_r_req_msg(ctx, receiver_id, sender_id, concat_req_msg.clone());
        }
    } else if let Some(discovery_msg) = msg.downcast_ref::<DiscoveryMessage>() {
        receive_discovery_msg(ctx, receiver_id, discovery_msg.clone());
    } else {
        log::warn(
            ctx,
            format!("Wrong message type received {msg:?} from {sender_id}"),
        );
    }
}

/// Checks if both peers have the same knowledge of the network
/// Returns whether the message should proceed or not
pub fn operation_msg_hash_guard(
    ctx: &mut Context,
    sender_id: usize,
    receiver_id: usize,
    sender_hash: u64,
) -> bool {
    let Ok(peer) = get_peer_of_type!(ctx, receiver_id, PGlmPeer) else {
        return false;
    };

    let receiver_hash = peer.state.hash;

    if receiver_hash == sender_hash {
        true
    } else {
        log::warn(
            ctx,
            format!(
                "receiver peer {receiver_id} has a different hash than sender peer {sender_id}, {receiver_hash} != {sender_hash}"
            ),
        );

        send_discovery_msg(ctx, sender_id, receiver_id);
        false
    }
}
