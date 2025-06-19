use crate::internal::core::{Context, Message, log, macros::get_peer_of_type};

use super::{algorithms, message::FlowUpdatingPairwiseMessage, peer::FlowUpdatingPairwisePeer};

pub fn on_message_receive(
    ctx: &mut Context,
    sender_id: usize,
    receiver_id: usize,
    msg: &dyn Message,
) {
    log::trace(
        ctx,
        format!("Peer_{receiver_id} received message from Peer_{sender_id}"),
    );

    let peer: &mut FlowUpdatingPairwisePeer =
        get_peer_of_type!(ctx, receiver_id, FlowUpdatingPairwisePeer).expect("peer should exist");

    if let Some(example_msg) = msg.downcast_ref::<FlowUpdatingPairwiseMessage>() {
        peer.estimates
            .insert(example_msg.sender, example_msg.estimate);
        peer.flows.insert(example_msg.sender, -example_msg.flow);

        algorithms::avg_and_send(ctx, receiver_id, example_msg.sender);
    }
}
