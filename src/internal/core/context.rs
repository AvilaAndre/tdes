use std::{cmp::Reverse, collections::BinaryHeap};

use indexmap::IndexMap;
use rand::{Rng, SeedableRng};

use ordered_float::OrderedFloat;
use rand_chacha::ChaCha8Rng;

use super::log::{Logger, LoggerLevel};
use super::options::ArrivalTimeCallback;
use super::{builtins, events::EventType, peer::CustomPeer};

pub type MessageDelayCallback = fn(&mut Context, usize, usize) -> Option<OrderedFloat<f64>>;
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
}
