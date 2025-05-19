pub mod example_peer;
pub mod flow_updating_peer;
pub mod internal;

use flow_updating_peer::{FlowUpdatingPairwiseMessage, FlowUpdatingPairwisePeer};
use internal::{context::Context, events::types::EventType, message_passing::send_message_to};
use rand::{Rng, distr::Uniform};

fn main() {
    let mut ctx = Context::new(0);

    let val1 = ctx.rng.sample(Uniform::new(0, 80).unwrap());
    let val2 = ctx.rng.sample(Uniform::new(0, 80).unwrap());

    let peer1 = ctx.add_peer(Box::new(FlowUpdatingPairwisePeer::new(0.35, 0.0, 0.0, val1)));
    let peer2 = ctx.add_peer(Box::new(FlowUpdatingPairwisePeer::new(0.0, 1.0, 0.0, val2)));

    /*
    ctx.add_event(SampleEvent::create(OrderedFloat(0.0), 1));
    ctx.add_event(TimerEvent::create(OrderedFloat(0.8)));
    ctx.add_event(MessageDeliveryEvent::create(OrderedFloat(5.0), 0, 1));
    */

    let msg = FlowUpdatingPairwiseMessage {
        sender: peer1,
        flow: 0.0,
        estimate: 0.0,
    };

    send_message_to(&mut ctx, peer1, peer2, Some(Box::new(msg)));

    // println!("{:?}", ctx.event_q.clone().into_sorted_vec());

    ctx.run();

    println!("Finished with clock {}", ctx.clock)
}
