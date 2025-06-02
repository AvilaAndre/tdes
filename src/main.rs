pub mod internal;
pub mod simulations;

use std::io;

use internal::core::{Context, simulation::SimulationRegistry};
use simulations::{DistributedGeneralizedLinearModel, Example, FlowUpdatingPairwise};

fn main() {
    let mut registry = SimulationRegistry::default();
    registry
        .register::<DistributedGeneralizedLinearModel>()
        .register::<FlowUpdatingPairwise>()
        .register::<Example>();


    let mut ctx = Context::new(Some(559464190120120835));

    let mut input = String::new();
    println!("Please select one of the following:");
    for sim in registry.list_simulations().iter() {
        println!("> {} - {}", sim.0, sim.1)
    }
    println!();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();

    println!("You entered: {}", input);

    if let Err(err) = registry.run_simulation(&input, &mut ctx) {
        println!("Simulation not run: {:?}", err);
    }

    println!("Finished with clock {}", ctx.clock)
}
