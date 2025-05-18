use std::{cmp::Reverse, collections::BinaryHeap};

use ordered_float::OrderedFloat;

use crate::EventType;
use crate::internal::events::Event;

pub struct Context {
    pub event_q: BinaryHeap<Reverse<EventType>>,
    pub clock: OrderedFloat<f64>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            event_q: BinaryHeap::new(),
            clock: OrderedFloat(0.0),
        }
    }

    pub fn add_event(&mut self, event: EventType) {
        self.event_q.push(Reverse(event));
    }

    pub fn run(&mut self) {
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
