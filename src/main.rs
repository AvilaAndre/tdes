use example_peer::ExamplePeer;

pub mod example_peer;
pub mod internal;

use internal::{
    context::Context, events::types::EventType, message_passing::send_message_to, peer::CustomPeer,
};

fn main() {
    let mut ctx = Context::new();

    let peer1: Box<dyn CustomPeer> = Box::new(ExamplePeer::new(1.0, 1.0, 0.0));
    let peer2: Box<dyn CustomPeer> = Box::new(ExamplePeer::new(-1.0, 1.0, 0.0));

    ctx.add_peer(peer1);
    ctx.add_peer(peer2);

    /*
    ctx.add_event(SampleEvent::create(OrderedFloat(0.0), 1));
    ctx.add_event(TimerEvent::create(OrderedFloat(0.8)));
    ctx.add_event(MessageDeliveryEvent::create(OrderedFloat(5.0), 0, 1));
    */

    send_message_to(&mut ctx, 0, 1);

    println!("{:?}", ctx.event_q.clone().into_sorted_vec());

    ctx.run();

    println!("Finished with clock {}", ctx.clock)
}
