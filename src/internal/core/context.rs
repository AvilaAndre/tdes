use std::{cmp::Reverse, collections::BinaryHeap};

use indexmap::IndexMap;
use rand::{Rng, SeedableRng};

use ordered_float::OrderedFloat;
use rand_chacha::ChaCha8Rng;

use crate::internal::core::log;

use super::log::{Logger, LoggerLevel};
use super::options::ArrivalTimeCallback;
use super::{
    builtins,
    events::{Event, EventType},
    peer::CustomPeer,
};

pub type MessageDelayCallback =
    fn(ctx: &mut Context, from: usize, to: usize) -> Option<OrderedFloat<f64>>;
pub type CustomHook = Box<dyn Fn(&mut Context)>;

pub struct Context {
    pub event_q: BinaryHeap<Reverse<EventType>>,
    pub clock: OrderedFloat<f64>,
    pub peers: Vec<Box<dyn CustomPeer>>,
    // Rust's HashMap is non-deterministic.
    pub links: Vec<IndexMap<usize, Option<f64>>>,
    pub rng: ChaCha8Rng,
    pub seed: u64,
    pub message_delay_cb: MessageDelayCallback,
    pub on_simulation_finish_hook: Option<CustomHook>,
    pub logger: Logger,
}

impl Context {
    #[must_use]
    pub fn new(seed_opt: Option<u64>, logger_level: Option<LoggerLevel>) -> Self {
        // Generate seed if none is provided
        let seed: u64 = match seed_opt {
            Some(s) => s,
            None => ChaCha8Rng::from_os_rng().random::<u64>(),
        };

        Self {
            event_q: BinaryHeap::new(),
            clock: OrderedFloat(0.0),
            peers: Vec::new(),
            links: Vec::new(),
            rng: ChaCha8Rng::seed_from_u64(seed),
            seed,
            message_delay_cb: builtins::arrival_time::ConstantArrivalTime::callback,
            on_simulation_finish_hook: None,
            logger: Logger::new(logger_level),
        }
    }

    #[must_use]
    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn add_event(&mut self, event: EventType) {
        self.event_q.push(Reverse(event));
    }

    pub fn add_peer(&mut self, mut custom_peer: Box<dyn CustomPeer>) -> usize {
        let new_id = self.peers.len();
        custom_peer.as_mut().get_peer_mut().id = Some(new_id);

        self.peers.push(custom_peer);
        self.links.push(IndexMap::new());
        new_id
    }

    // Adds a link to another peer.
    // If latency is provided, that value will always be used,
    // if not, the simulator will calculate it using "message_delay_cb".
    pub fn add_oneway_link(&mut self, from: usize, to: usize, latency: Option<f64>) {
        if from < self.links.len() && to < self.links.len() {
            self.links[from].insert(to, latency);
        } else {
            log::global_warn(format!(
                "Failed to create a one way link between peers {from} and {to} because at least one of them does not exist."
            ));
        }
    }

    // Adds two links between two neighbors with the same latency.
    // If latency is provided, that value will always be used,
    // if not, the simulator will calculate it using "message_delay_cb".
    pub fn add_twoway_link(&mut self, from: usize, to: usize, latency: Option<f64>) {
        if from < self.links.len() && to < self.links.len() {
            self.links[from].insert(to, latency);
            self.links[to].insert(from, latency);
        } else {
            log::global_warn(format!(
                "Failed to create a two way link between peers {from} and {to} because at least one of them does not exist."
            ));
        }
    }

    // TODO: Move this out of Context struct
    pub fn get_neighbors(&mut self, peer_id: usize) -> Option<Vec<usize>> {
        Some(
            self.links
                .get(peer_id)?
                .keys()
                .copied()
                .collect::<Vec<usize>>(),
        )
    }

    pub fn run(&mut self, deadline_opt: Option<f64>) {
        log::global_internal("STARTING SIMULATION");
        log::internal(self, "SIMULATION STARTED");

        let (has_deadline, deadline) = match deadline_opt {
            Some(dedln) => (dedln >= 0.0, OrderedFloat(dedln)),
            None => (false, OrderedFloat(0.0)),
        };

        while !self.event_q.is_empty() {
            // TODO: Deal with this unwrap
            let mut ev = self.event_q.pop().unwrap().0;

            // Do not process events after the deadline
            if has_deadline && ev.timestamp() > deadline {
                self.clock = deadline;
                log::global_internal("The simulation reached the deadline");
                break;
            }

            if ev.timestamp() < self.clock {
                log::global_error("An event was earlier than the simulation clock");
            }

            self.clock = ev.timestamp();

            ev.process(self);
        }

        if let Some(hook) = self.on_simulation_finish_hook.take() {
            hook(self);
        }

        log::internal(self, "SIMULATION FINISHED");
        log::global_internal(format!(
            "FINISHED SIMULATION'S SEED IS \"{:?}\"",
            self.seed()
        ));

        self.logger.close_log_file();
        log::global_internal(
            "Log file closed, will not be written anymore unless a new log file is specified.",
        );
        self.logger.close_metrics_file();
        log::global_internal(
            "Metrics file closed, will not be written anymore unless a new metrics file is specified.",
        );
    }
}
