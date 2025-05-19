pub mod example_peer;
pub mod flow_updating_peer;
pub mod internal;

use flow_updating_peer::{FlowUpdatingPairwiseMessage, FlowUpdatingPairwisePeer};
use internal::{context::Context, events::types::EventType, message_passing::send_message_to};
use ordered_float::OrderedFloat;
use rand::{Rng, distr::Uniform};

fn main() {
    let mut ctx = Context::new(Some(559464190120120835));

    let val1 = ctx.rng.sample(Uniform::new(0, 80).unwrap());
    let val2 = ctx.rng.sample(Uniform::new(0, 80).unwrap());
    let val3 = ctx.rng.sample(Uniform::new(0, 80).unwrap());

    let peer1 = ctx.add_peer(Box::new(FlowUpdatingPairwisePeer::new(
        0.35, 0.0, 0.0, val1,
    )));
    let peer2 = ctx.add_peer(Box::new(FlowUpdatingPairwisePeer::new(0.0, 1.0, 0.0, val2)));
    let peer3 = ctx.add_peer(Box::new(FlowUpdatingPairwisePeer::new(0.0, 0.3, 0.0, val3)));

    /*
    ctx.add_event(SampleEvent::create(OrderedFloat(0.0), 1));
    ctx.add_event(TimerEvent::create(OrderedFloat(0.8)));
    ctx.add_event(MessageDeliveryEvent::create(OrderedFloat(5.0), 0, 1));
    */

    let msg1 = FlowUpdatingPairwiseMessage {
        sender: peer1,
        flow: 0.0,
        estimate: 0.0,
    };
    let msg2 = FlowUpdatingPairwiseMessage {
        sender: peer1,
        flow: 0.0,
        estimate: 0.0,
    };
    let msg3 = FlowUpdatingPairwiseMessage {
        sender: peer2,
        flow: 0.0,
        estimate: 0.0,
    };

    send_message_to(&mut ctx, peer1, peer2, Some(Box::new(msg1)));
    send_message_to(&mut ctx, peer1, peer3, Some(Box::new(msg2)));
    send_message_to(&mut ctx, peer2, peer3, Some(Box::new(msg3)));

    // println!("{:?}", ctx.event_q.clone().into_sorted_vec());

    ctx.run_for(OrderedFloat(5.0));

    println!("Finished with clock {}", ctx.clock)
}
