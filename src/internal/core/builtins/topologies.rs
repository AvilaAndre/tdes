use crate::internal::core::{
    Context, engine,
    experiment::ConnectionInfo,
    log::{self},
    macros::define_custom_topology,
    options::Topology,
};

fn onewaycustomtopology(
    ctx: &mut Context,
    _n_peers: usize,
    custom_list: Option<Vec<ConnectionInfo>>,
) {
    if let Some(list) = custom_list {
        for (from, to, info) in list {
            engine::add_oneway_link(ctx, from, to, info);
        }
    } else {
        log::global_warn(
            "Cannot apply 'onewaycustom' topology because no 'connections' was supplied",
        );
    }
}

fn twowaycustom_topology(
    ctx: &mut Context,
    _n_peers: usize,
    custom_list: Option<Vec<ConnectionInfo>>,
) {
    if let Some(list) = custom_list {
        for (from, to, info) in list {
            engine::add_twoway_link(ctx, from, to, info);
        }
    } else {
        log::global_warn(
            "Cannot apply 'twowaycustom' topology because no 'connections' was supplied",
        );
    }
}

fn full_topology(ctx: &mut Context, n_peers: usize, _custom_list: Option<Vec<ConnectionInfo>>) {
    for i in 0..n_peers {
        for j in i + 1..n_peers {
            engine::add_twoway_link(ctx, i, j, None);
        }
    }
}

fn star_topology(ctx: &mut Context, n_peers: usize, _custom_list: Option<Vec<ConnectionInfo>>) {
    let center_idx = 0;
    for i in 1..n_peers {
        engine::add_twoway_link(ctx, i, center_idx, None);
    }
}

fn ring_topology(ctx: &mut Context, n_peers: usize, _custom_list: Option<Vec<ConnectionInfo>>) {
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
