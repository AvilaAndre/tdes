use crate::internal::core::Context;

pub fn full(ctx: &mut Context, n_peers: usize) {
    // full connection
    for i in 0..n_peers {
        for j in i + 1..n_peers {
            ctx.add_twoway_link(i, j, None);
        }
    }
}

pub fn star(ctx: &mut Context, n_peers: usize, center_idx: usize) {
    for i in 0..n_peers {
        if i == center_idx {
            continue;
        }
        ctx.add_twoway_link(i, center_idx, None);
    }
}
