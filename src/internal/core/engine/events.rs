use crate::internal::core::{
    Context,
    events::{Event, EventType, Timer, TimerEvent},
    hooks::SimulationHooks,
    log,
};

use ordered_float::OrderedFloat;

pub fn add_event(ctx: &mut Context, event: EventType) {
    ctx.push_event(event);
}

pub fn add_timer(ctx: &mut Context, time: OrderedFloat<f64>, timer: impl Timer + 'static) {
    ctx.push_event(EventType::TimerEvent(TimerEvent::new(
        time,
        Box::new(timer),
    )));
}

pub fn run(ctx: &mut Context, hooks: &SimulationHooks, deadline_opt: Option<f64>) {
    log::global_internal("STARTING SIMULATION LOOP");
    log::internal(ctx, "SIMULATION LOOP STARTED");

    let (has_deadline, deadline) = match deadline_opt {
        Some(dedln) => (dedln >= 0.0, OrderedFloat(dedln)),
        None => (false, OrderedFloat(0.0)),
    };

    while let Some(mut ev) = ctx.get_next_event() {
        // Do not process events after the deadline
        if has_deadline && ev.timestamp() > deadline {
            ctx.clock = deadline;
            log::global_internal(format!("The simulation reached the deadline: {deadline}"));
            break;
        }

        if ev.timestamp() < ctx.clock {
            log::global_error("An event's timestamp was earlier than the simulation clock");
            continue;
        }

        ctx.clock = ev.timestamp();

        ev.process(ctx);

        if (hooks.finish_condition)(ctx) {
            break;
        }
    }

    log::internal(ctx, "SIMULATION LOOP FINISHED");

    (hooks.on_simulation_finish)(ctx);

    log::global_internal(format!("FINISHED SIMULATION, SEED IS \"{:?}\"", ctx.seed()));

    ctx.logger.close_log_file();
    log::global_internal(
        "Log file closed, will not be written anymore unless a new log file is specified.",
    );
    ctx.logger.close_metrics_file();
    log::global_internal(
        "Metrics file closed, will not be written anymore unless a new metrics file is specified.",
    );
}
