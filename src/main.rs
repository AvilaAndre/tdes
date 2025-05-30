pub mod internal;

pub mod simulations;

use internal::{context::Context, events::types::EventType};
use simulations::distributed_generalized_linear_model;

fn main() {
    let mut ctx = Context::new(Some(559464190120120835));
    // flow_updating_pairwise::start(&mut ctx);

    distributed_generalized_linear_model::start(&mut ctx);

    println!("Finished with clock {}", ctx.clock)
}
