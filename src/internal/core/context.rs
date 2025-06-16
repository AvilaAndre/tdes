use std::{cmp::Reverse, collections::BinaryHeap};

use indexmap::IndexMap;
use ordered_float::OrderedFloat;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::num_traits::Zero;

use super::{
    builtins, distributions,
    events::EventType,
    experiment::{Jitter, LinkInfo},
    log,
    log::{Logger, LoggerLevel},
    options::ArrivalTimeCallback,
    peer::CustomPeer,
};

pub type MessageDelayCallback = fn(&mut Context, usize, usize) -> Option<OrderedFloat<f64>>;

pub struct Context {
    pub event_q: BinaryHeap<Reverse<EventType>>,
    pub clock: OrderedFloat<f64>,
    pub peers: Vec<Box<dyn CustomPeer>>,
    // Rust's HashMap is non-deterministic.
    pub links: Vec<IndexMap<usize, LinkInfo>>,
    pub rng: ChaCha8Rng,
    pub seed: u64,
    pub message_delay_cb: MessageDelayCallback,
    pub logger: Logger,
    drop_rate: f64,
    duplicate_rate: f64,
    jitter: Jitter,
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
            message_delay_cb: builtins::arrival_times::ConstantArrivalTime::callback,
            logger: Logger::new(logger_level),
            drop_rate: 0.0,
            duplicate_rate: 0.0,
            jitter: Jitter::default(),
        }
    }

    #[must_use]
    pub fn seed(&self) -> u64 {
        self.seed
    }

    #[inline]
    #[must_use]
    pub fn get_drop_rate(&self) -> f64 {
        self.drop_rate
    }

    #[inline]
    pub fn set_drop_rate(&mut self, new_rate: f64) {
        if !(0.0..=1.0).contains(&new_rate) {
            log::global_warn(format!(
                "Drop rate should be between 0.0 and 1.0, not {new_rate}."
            ));
        }

        self.drop_rate = new_rate.clamp(0.0, 1.0);
    }

    #[inline]
    #[must_use]
    pub fn get_duplicate_rate(&self) -> f64 {
        self.duplicate_rate
    }

    #[inline]
    pub fn set_duplicate_rate(&mut self, new_rate: f64) {
        if !(0.0..=1.0).contains(&new_rate) {
            log::global_warn(format!(
                "Duplicate rate should be between 0.0 and 1.0, not {new_rate}."
            ));
        }

        self.duplicate_rate = new_rate.clamp(0.0, 1.0);
    }

    /// Returns a jitter value by sampling from
    /// self.jitter.distribution and multiplying
    /// it by self.jitter.multiplier.
    #[inline]
    pub fn get_jitter_value(&mut self) -> OrderedFloat<f64> {
        if self.jitter.multiplier.is_zero() {
            return OrderedFloat(0.0);
        }

        let from_sample =
            distributions::get_value(self, self.jitter.distribution).unwrap_or(OrderedFloat(0.0));

        from_sample * self.jitter.multiplier
    }

    #[inline]
    pub fn set_jitter(&mut self, jitter: Jitter) {
        self.jitter = jitter;
    }

    #[inline]
    pub fn get_next_event(&mut self) -> Option<EventType> {
        self.event_q.pop().map(|Reverse(ev)| ev)
    }
}
