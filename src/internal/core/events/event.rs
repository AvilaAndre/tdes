use enum_dispatch::enum_dispatch;
use ordered_float::OrderedFloat;

use super::Context;

#[enum_dispatch]
pub trait Event {
    fn id(&self) -> u64;
    fn set_id(&mut self, id: u64);
    fn timestamp(&self) -> OrderedFloat<f64>;
    fn process(&mut self, ctx: &mut Context);
}

macro_rules! impl_timestamp_id_ordering {
    ($struct_name:ident) => {
        impl PartialOrd for $struct_name {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $struct_name {
            fn cmp(&self, other: &Self) -> Ordering {
                // First compare by timestamp
                match self.timestamp.total_cmp(&other.timestamp) {
                    Ordering::Equal => {
                        // If timestamps are equal, compare by id
                        self.id.cmp(&other.id)
                    }
                    other_ordering => other_ordering,
                }
            }
        }

        impl PartialEq for $struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.timestamp == other.timestamp && self.id == other.id
            }
        }

        impl Eq for $struct_name {}
    };
}

pub(crate) use impl_timestamp_id_ordering;
