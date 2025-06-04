use crate::internal::core::{Context, engine, log, options::Topology};

macro_rules! define_custom_topology {
    ($name:ident, $topology_name:expr, $connect_fn:path) => {
        pub struct $name;

        impl Topology for $name {
            fn name() -> &'static str {
                $topology_name
            }

            fn connect(
                ctx: &mut Context,
                n_peers: usize,
                custom_list: Option<Vec<(usize, usize, Option<f64>)>>,
            ) {
                $connect_fn(ctx, n_peers, custom_list);
            }
        }
    };
}

fn onewaycustomtopology(
    ctx: &mut Context,
    _n_peers: usize,
    custom_list: Option<Vec<(usize, usize, Option<f64>)>>,
) {
    if let Some(list) = custom_list {
        for (from, to, latency) in list {
            engine::add_oneway_link(ctx, from, to, latency);
        }
    } else {
        log::global_warn(
            "Cannot apply 'onewaycustom' topology because no 'custom_list' was supplied",
        );
    }
}

fn twowaycustom_topology(
    ctx: &mut Context,
    _n_peers: usize,
    custom_list: Option<Vec<(usize, usize, Option<f64>)>>,
) {
    if let Some(list) = custom_list {
        for (from, to, latency) in list {
            engine::add_twoway_link(ctx, from, to, latency);
        }
    } else {
        log::global_warn(
            "Cannot apply 'twowaycustom' topology because no 'custom_list' was supplied",
        );
    }
}

fn full_topology(
    ctx: &mut Context,
    n_peers: usize,
    _custom_list: Option<Vec<(usize, usize, Option<f64>)>>,
) {
    for i in 0..n_peers {
        for j in i + 1..n_peers {
            engine::add_twoway_link(ctx, i, j, None);
        }
    }
}

fn star_topology(
    ctx: &mut Context,
    n_peers: usize,
    _custom_list: Option<Vec<(usize, usize, Option<f64>)>>,
) {
    let center_idx = 0;
    for i in 1..n_peers {
        engine::add_twoway_link(ctx, i, center_idx, None);
    }
}

fn ring_topology(
    ctx: &mut Context,
    n_peers: usize,
    _custom_list: Option<Vec<(usize, usize, Option<f64>)>>,
) {
    for i in 1..n_peers {
        engine::add_twoway_link(ctx, i - 1, i, None);
    }
    engine::add_twoway_link(ctx, n_peers - 1, 0, None);
}

define_custom_topology!(OneWayCustomTopology, "onewaycustom", onewaycustomtopology);
define_custom_topology!(TwoWayCustomTopology, "twowaycustom", twowaycustom_topology);
define_custom_topology!(FullTopology, "full", full_topology);
define_custom_topology!(StarTopology, "star", star_topology);
define_custom_topology!(RingTopology, "ring", ring_topology);
