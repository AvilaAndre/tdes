pub mod internal;

pub mod simulations;

use internal::{context::Context, events::types::EventType};
use simulations::flow_updating_pairwise;

fn main() {
    let mut ctx = Context::new(Some(559464190120120835));
    flow_updating_pairwise::start(&mut ctx);

    println!("Finished with clock {}", ctx.clock)
}
