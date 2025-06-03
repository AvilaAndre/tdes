use crate::internal::core::{Context, options::Topology};

macro_rules! define_custom_topology {
    ($name:ident, $topology_name:expr, |$ctx:ident, $n_peers:ident| $connect_fn:block) => {
        pub struct $name;

        impl Topology for $name {
            fn name() -> &'static str
            where
                Self: Sized,
            {
                $topology_name
            }

            fn connect($ctx: &mut Context, $n_peers: usize) $connect_fn
        }
    };
}

define_custom_topology!(FullTopology, "full", |ctx, n_peers| {
    for i in 0..n_peers {
        for j in i + 1..n_peers {
            ctx.add_twoway_link(i, j, None);
        }
    }
});

define_custom_topology!(StarTopology, "star", |ctx, n_peers| {
    let center_idx = 0;
    for i in 1..n_peers {
        ctx.add_twoway_link(i, center_idx, None);
    }
});

define_custom_topology!(RingTopology, "ring", |ctx, n_peers| {
    for i in 1..n_peers {
        ctx.add_twoway_link(i - 1, i, None);
    }

    ctx.add_twoway_link(n_peers - 1, 0, None);
});
