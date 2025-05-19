use std::{cmp::Reverse, collections::BinaryHeap};

use ordered_float::OrderedFloat;

use crate::EventType;
use crate::internal::events::Event;

use super::peer::CustomPeer;

pub struct Context {
    pub event_q: BinaryHeap<Reverse<EventType>>,
    pub clock: OrderedFloat<f64>,
    // TODO: use sparse set if peers can be removed
    pub peers: Vec<Box<dyn CustomPeer>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            event_q: BinaryHeap::new(),
            clock: OrderedFloat(0.0),
            peers: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: EventType) {
        self.event_q.push(Reverse(event));
    }

    pub fn add_peer(&mut self, mut custom_peer: Box<dyn CustomPeer>) -> usize {
        let new_id = self.peers.len();
        custom_peer.as_mut().get_peer_mut().id = Some(new_id);

        println!(
            "Adding peer with id {:?} on position {:?}",
            custom_peer.get_peer().id,
            custom_peer.get_peer().position
        );
        self.peers.push(custom_peer);

        new_id
    }

    pub fn run(&mut self) {
        println!(">> STARTING SIMULATION");
        while !self.event_q.is_empty() {
            let ev = self.event_q.pop().unwrap().0;

            if ev.timestamp() < self.clock {
                panic!("An event was earlier than the simulation clock");
            }

            self.clock = ev.timestamp();

            // println!("{:?}", ev);

            ev.process(self);
        }
        println!(">> FINISHED SIMULATION");
    }
}
