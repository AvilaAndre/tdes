use example_peer::{ExampleMessage, ExamplePeer};

pub mod example_peer;
pub mod internal;

use internal::{context::Context, events::types::EventType, message_passing::send_message_to};

fn main() {
    let mut ctx = Context::new();

    let peer1 = ctx.add_peer(Box::new(ExamplePeer::new(0.0, 1.0, 0.0)));
    let peer2 = ctx.add_peer(Box::new(ExamplePeer::new(-0.5, 1.0, 0.0)));

    /*
    ctx.add_event(SampleEvent::create(OrderedFloat(0.0), 1));
    ctx.add_event(TimerEvent::create(OrderedFloat(0.8)));
    ctx.add_event(MessageDeliveryEvent::create(OrderedFloat(5.0), 0, 1));
    */

    let msg = ExampleMessage {
        sender: peer1,
        receiver: peer2,
    };

    send_message_to(&mut ctx, peer1, peer2, Some(Box::new(msg)));

    // println!("{:?}", ctx.event_q.clone().into_sorted_vec());

    ctx.run();

    println!("Finished with clock {}", ctx.clock)
}
