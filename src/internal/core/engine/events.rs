use crate::internal::core::{
    Context,
    events::{Event, EventType},
    log,
};

use ordered_float::OrderedFloat;
use std::cmp::Reverse;

pub fn add_event(ctx: &mut Context, event: EventType) {
    ctx.event_q.push(Reverse(event));
}

pub fn run(ctx: &mut Context, deadline_opt: Option<f64>) {
    log::global_internal("STARTING SIMULATION");
    log::internal(ctx, "SIMULATION STARTED");

    let (has_deadline, deadline) = match deadline_opt {
        Some(dedln) => (dedln >= 0.0, OrderedFloat(dedln)),
        None => (false, OrderedFloat(0.0)),
    };

    while !ctx.event_q.is_empty() {
        // TODO: Deal with this unwrap
        let mut ev = ctx.event_q.pop().unwrap().0;

        // Do not process events after the deadline
        if has_deadline && ev.timestamp() > deadline {
            ctx.clock = deadline;
            log::global_internal("The simulation reached the deadline");
            break;
        }

        if ev.timestamp() < ctx.clock {
            log::global_error("An event was earlier than the simulation clock");
        }

        ctx.clock = ev.timestamp();

        ev.process(ctx);
    }

    if let Some(hook) = ctx.on_simulation_finish_hook.take() {
        hook(ctx);
    }

    log::internal(ctx, "SIMULATION FINISHED");
    log::global_internal(format!(
        "FINISHED SIMULATION'S SEED IS \"{:?}\"",
        ctx.seed()
    ));

    ctx.logger.close_log_file();
    log::global_internal(
        "Log file closed, will not be written anymore unless a new log file is specified.",
    );
    ctx.logger.close_metrics_file();
    log::global_internal(
        "Metrics file closed, will not be written anymore unless a new metrics file is specified.",
    );
}
