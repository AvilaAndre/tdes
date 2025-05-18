use ordered_float::OrderedFloat;

pub mod internal;

use internal::{
    context::Context,
    events::types::{
        EventType, message_delivery::MessageDeliveryEvent, sample::SampleEvent, timer::TimerEvent,
    },
    peer::Peer,
};

fn main() {
    let mut ctx = Context::new();

    let peer1 = Peer::new(1.0, 1.0, 0.0);
    let peer2 = Peer::new(-1.0, 1.0, 0.0);

    ctx.add_peer(peer1);
    ctx.add_peer(peer2);

    let (ev1, ev2) = (
        SampleEvent::create(OrderedFloat(0.0001), 7),
        SampleEvent::create(OrderedFloat(0.0001), 3),
    );

    ctx.add_event(ev1);
    ctx.add_event(ev2);
    ctx.add_event(SampleEvent::create(OrderedFloat(0.0), 1));
    ctx.add_event(SampleEvent::create(OrderedFloat(1.2), 2));
    ctx.add_event(SampleEvent::create(OrderedFloat(0.5), 3));

    ctx.add_event(TimerEvent::create(OrderedFloat(0.075)));
    ctx.add_event(TimerEvent::create(OrderedFloat(0.8)));

    ctx.add_event(MessageDeliveryEvent::create(OrderedFloat(5.0), 0, 1));

    println!("{:?}", ctx.event_q.clone().into_sorted_vec());

    ctx.run();

    println!("Finished with clock {}", ctx.clock)
}
