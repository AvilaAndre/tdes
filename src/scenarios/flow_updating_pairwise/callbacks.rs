use crate::internal::core::{Context, Message, log, macros::get_peer_of_type};

use super::{algorithms, message::FlowUpdatingPairwiseMessage, peer::FlowUpdatingPairwisePeer};

pub fn example_on_message_receive(ctx: &mut Context, receiver_id: usize, msg: Box<dyn Message>) {
    log::trace(
        ctx,
        format!("FlowUpdatingPairwisePeer {receiver_id} received a message"),
    );

    let peer: &mut FlowUpdatingPairwisePeer =
        get_peer_of_type!(ctx, receiver_id, FlowUpdatingPairwisePeer).expect("peer should exist");

    if let Some(example_msg) = msg.downcast_ref::<FlowUpdatingPairwiseMessage>() {
        peer.estimates
            .insert(example_msg.sender, example_msg.estimate);
        peer.flows.insert(example_msg.sender, -example_msg.flow);

        // This is not part of the algorithm
        /*
        if (peer.last_avg - example_msg.estimate).abs() < 0.00001 {
            return;
        }
        */

        algorithms::avg_and_send(ctx, receiver_id, example_msg.sender);
    }
}
