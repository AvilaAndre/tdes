use ordered_float::OrderedFloat;

pub mod internal;

use internal::{
    context::Context,
    events::types::{EventType, sample::SampleEvent, timer::TimerEvent},
};

fn main() {
    let mut ctx = Context::new();

    let ev1 = SampleEvent::new(OrderedFloat(0.0001), 7);

    ctx.add_event(EventType::SampleEvent(ev1));

    ctx.add_event(EventType::SampleEvent(SampleEvent::new(
        OrderedFloat(0.0),
        1,
    )));

    ctx.add_event(EventType::SampleEvent(SampleEvent::new(
        OrderedFloat(1.2),
        2,
    )));

    ctx.add_event(EventType::SampleEvent(SampleEvent::new(
        OrderedFloat(0.5),
        3,
    )));

    ctx.add_event(EventType::TimerEvent(TimerEvent::new(OrderedFloat(0.8))));

    println!("{:?}", ctx.event_q.clone().into_sorted_vec());

    ctx.run();

    println!("Finished with clock {}", ctx.clock)
}
