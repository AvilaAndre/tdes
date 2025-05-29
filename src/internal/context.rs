use std::{cmp::Reverse, collections::BinaryHeap};

use rand::{Rng, SeedableRng};

use ordered_float::OrderedFloat;
use rand_chacha::ChaCha8Rng;

use crate::EventType;
use crate::internal::events::Event;

use super::message_passing::distance_based_arrival_time;
use super::peer::CustomPeer;

type MessageDelayFn = fn(ctx: &mut Context, from: usize, to: usize) -> OrderedFloat<f64>;

pub struct Context {
    pub event_q: BinaryHeap<Reverse<EventType>>,
    pub clock: OrderedFloat<f64>,
    // TODO: use sparse set if peers can be removed
    pub peers: Vec<Box<dyn CustomPeer>>,
    pub rng: ChaCha8Rng,
    pub seed: u64,
    pub message_delay: MessageDelayFn,
}

impl Context {
    pub fn new(seed_opt: Option<u64>) -> Self {
        // Generate seed if none is provided
        let seed: u64 = match seed_opt {
            Some(s) => s,
            None => ChaCha8Rng::from_os_rng().random::<u64>(),
        };

        Self {
            event_q: BinaryHeap::new(),
            clock: OrderedFloat(0.0),
            peers: Vec::new(),
            rng: ChaCha8Rng::seed_from_u64(seed),
            seed,
            message_delay: distance_based_arrival_time,
        }
    }

    pub fn seed(&self) -> u64 {
        self.seed
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
        self.run_for(OrderedFloat(-1.0));
    }

    pub fn run_for(&mut self, deadline: OrderedFloat<f64>) {
        println!(">> STARTING SIMULATION");

        let has_deadline = deadline >= OrderedFloat(0.0);

        while !self.event_q.is_empty() {
            let mut ev = self.event_q.pop().unwrap().0;

            // Do not process events after the deadline
            if has_deadline && ev.timestamp() > deadline {
                self.clock = deadline;
                println!("Simulation reached the deadline");
                break;
            }

            if ev.timestamp() < self.clock {
                panic!("An event was earlier than the simulation clock");
            }

            self.clock = ev.timestamp();

            ev.process(self);
        }

        println!("Finished simulation with seed \"{:?}\".", self.seed());
        println!(">> FINISHED SIMULATION");
    }
}
