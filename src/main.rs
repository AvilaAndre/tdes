use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

use ordered_float::OrderedFloat;

struct Context {
    pub event_q: BinaryHeap<Reverse<EventType>>,
    pub clock: OrderedFloat<f64>,
}

impl Context {
    fn new() -> Self {
        Self {
            event_q: BinaryHeap::new(),
            clock: OrderedFloat(0.0),
        }
    }

    fn add_event(&mut self, event: EventType) {
        self.event_q.push(Reverse(event));
    }

    fn run(&mut self) {
        while !self.event_q.is_empty() {
            let ev = self.event_q.pop().unwrap().0;

            if ev.timestamp() < self.clock {
                panic!("An event was earlier than the simulation clock");
            }

            self.clock = ev.timestamp();

            // println!("{:?}", ev);

            ev.trigger(self);
        }
    }
}

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

#[derive(Debug, Eq, Clone, Copy)]
pub enum EventType {
    SampleEvent(SampleEvent),
    TimerEvent(TimerEvent),
}

impl Ord for EventType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp().cmp(&other.timestamp())
    }
}
impl PartialOrd for EventType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EventType {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp() == other.timestamp()
    }
}

impl Event for EventType {
    fn timestamp(&self) -> OrderedFloat<f64> {
        let event: &dyn Event = match self {
            EventType::SampleEvent(event) => event,
            EventType::TimerEvent(event) => event,
        };
        event.timestamp()
    }

    fn trigger(&self, ctx: &mut Context) {
        let event: &dyn Event = match self {
            EventType::SampleEvent(event) => event,
            EventType::TimerEvent(event) => event,
        };
        event.trigger(ctx)
    }
}

trait Event {
    fn timestamp(&self) -> OrderedFloat<f64>;
    fn trigger(&self, ctx: &mut Context);
}

// types

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct SampleEvent {
    timestamp: OrderedFloat<f64>,
    value: u64,
}

impl SampleEvent {
    fn new(timestamp: OrderedFloat<f64>, value: u64) -> Self {
        Self { timestamp, value }
    }
}

impl Event for SampleEvent {
    fn timestamp(&self) -> OrderedFloat<f64> {
        self.timestamp
    }

    fn trigger(&self, ctx: &mut Context) {
        println!("[{}]: SampleEvent triggered with value {}!", ctx.clock, self.value);
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct TimerEvent {
    timestamp: OrderedFloat<f64>,
}

impl TimerEvent {
    fn new(timestamp: OrderedFloat<f64>) -> Self {
        Self { timestamp }
    }
}

impl Event for TimerEvent {
    fn timestamp(&self) -> OrderedFloat<f64> {
        self.timestamp
    }

    fn trigger(&self, ctx: &mut Context) {
        println!("[{}] TimerEvent triggered!", ctx.clock);
    }
}
