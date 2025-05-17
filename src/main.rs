use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

use ordered_float::OrderedFloat;

fn main() {
    let mut event_q: BinaryHeap<Reverse<EventType>> = BinaryHeap::new();

    event_q.push(Reverse(EventType::SampleEvent(SampleEvent::new(
        OrderedFloat(0.0),
        1,
    ))));
    event_q.push(Reverse(EventType::SampleEvent(SampleEvent::new(
        OrderedFloat(1.2),
        2,
    ))));
    event_q.push(Reverse(EventType::SampleEvent(SampleEvent::new(
        OrderedFloat(0.5),
        3,
    ))));
    event_q.push(Reverse(EventType::TimerEvent(TimerEvent::new(
        OrderedFloat(0.8),
    ))));

    println!("{:?}", event_q.clone().into_sorted_vec());

    let mut sim_clock = OrderedFloat(0.0);

    while !event_q.is_empty() {
        let ev = event_q.pop().unwrap().0;

        if ev.timestamp() < sim_clock {
            panic!("An event was earlier than the simulation clock");
        }

        sim_clock = ev.timestamp();

        // println!("{:?}", ev);

        ev.run();
    }

    println!("Finished with clock {}", sim_clock)
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

    fn run(&self) {
        let event: &dyn Event = match self {
            EventType::SampleEvent(event) => event,
            EventType::TimerEvent(event) => event,
        };
        event.run()
    }
}

trait Event {
    fn timestamp(&self) -> OrderedFloat<f64>;
    fn run(&self);
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

    fn run(&self) {
        println!("SampleEvent triggered with value {}!", self.value);
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

    fn run(&self) {
        println!("TimerEvent triggered!");
    }
}
