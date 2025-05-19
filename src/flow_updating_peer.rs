use std::collections::HashMap;

use crate::{
    get_peer_of_type,
    internal::{
        context::Context,
        events::types::timer::{Timer, TimerEvent},
        message::Message,
        message_passing::send_message_to,
        peer::{CustomPeer, Peer},
    },
};

pub struct FlowUpdatingPairwisePeer {
    pub peer: Peer,
    pub value: i32,
    pub flows: HashMap<usize, f64>,
    pub estimates: HashMap<usize, f64>,
    pub ticks_since_last_avg: HashMap<usize, u32>,
    pub last_avg: f64,
}

impl FlowUpdatingPairwisePeer {
    pub fn new(x: f64, y: f64, z: f64, value: i32) -> Self {
        println!("Instantiated FlowUpdatingPairwisePeer with value {}", value);
        Self {
            peer: {
                let mut this = Peer::new(x, y, z);
                this.on_message_receive = example_on_message_receive;
                this
            },
            value,
            flows: HashMap::new(),
            estimates: HashMap::new(),
            ticks_since_last_avg: HashMap::new(),
            last_avg: 0.0,
        }
    }
}

fn example_on_message_receive(
    ctx: &mut Context,
    receiver_id: usize,
    msg: Option<Box<dyn Message>>,
) {
    let peer: &mut FlowUpdatingPairwisePeer =
        get_peer_of_type!(ctx, receiver_id, FlowUpdatingPairwisePeer).expect("peer should exist");

    if let Some(boxed_msg) = msg {
        if let Some(example_msg) = boxed_msg.downcast_ref::<FlowUpdatingPairwiseMessage>() {
            peer.estimates
                .insert(example_msg.sender, example_msg.estimate);
            peer.flows.insert(example_msg.sender, -example_msg.flow);

            avg_and_send(ctx, receiver_id, example_msg.sender);
        }
    }
}

fn avg_and_send(ctx: &mut Context, peer_id: usize, neigh_id: usize) {
    let peer: &mut FlowUpdatingPairwisePeer =
        get_peer_of_type!(ctx, peer_id, FlowUpdatingPairwisePeer).expect("peer should exist");

    // TODO: Delete this debug line
    println!(
        "[{}] peer with id {} has value {}, last_avg is {}",
        ctx.clock, peer_id, peer.value, peer.last_avg
    );

    // FIXME: only values from neighbors
    let flows_sum: f64 = peer.flows.values().sum();
    let estimate = (peer.value as f64) - flows_sum;
    let avg = (peer.estimates.get(&neigh_id).copied().unwrap_or(0.0) + estimate) / 2.0;

    peer.last_avg = avg;
    peer.flows.insert(
        neigh_id,
        peer.flows.get(&neigh_id).copied().unwrap_or(0.0) + avg
            - peer.estimates.get(&neigh_id).copied().unwrap_or(0.0),
    );
    peer.estimates.insert(neigh_id, avg);

    peer.ticks_since_last_avg.insert(neigh_id, 0);

    let payload = Box::new(FlowUpdatingPairwiseMessage {
        sender: peer_id,
        flow: peer.flows.get(&neigh_id).copied().unwrap_or(0.0),
        estimate: avg,
    });

    send_message_to(ctx, peer_id, neigh_id, Some(payload));
}

// TODO: Implement tick() function with neighbors
fn tick(ctx: &mut Context, peer_id: usize) {
    //for neigh in self.neighbors.keys():
    //    if self.ticks_since_last_avg[neigh] < threshold:
    //        self.avg_and_send(neigh)

    let size = ctx.peers.len().clone();

    for neigh_id in 0..size {
        if neigh_id == peer_id {
            continue;
        }

        let peer: &mut FlowUpdatingPairwisePeer =
            get_peer_of_type!(ctx, peer_id, FlowUpdatingPairwisePeer).expect("peer should exist");

        let neigh_ticks = peer
            .ticks_since_last_avg
            .get(&neigh_id)
            .copied()
            .unwrap_or(0);

        // TODO: make number of ticks a const
        if neigh_ticks > 50 {
            avg_and_send(ctx, peer_id, neigh_id);
        } else {
            peer.ticks_since_last_avg.insert(neigh_id, neigh_ticks + 1);
        }
    }
}

impl CustomPeer for FlowUpdatingPairwisePeer {
    fn get_peer(&self) -> &Peer {
        &self.peer
    }

    fn get_peer_mut(&mut self) -> &mut Peer {
        &mut self.peer
    }
}

#[derive(Debug, Clone)]
pub struct FlowUpdatingPairwiseMessage {
    pub sender: usize,
    pub flow: f64,
    pub estimate: f64,
}
impl Message for FlowUpdatingPairwiseMessage {}

#[derive(Debug, Clone)]
pub struct TickTimer {
    pub interval: f64,
}

impl Timer for TickTimer {
    fn fire(&self, ctx: &mut Context) {
        ctx.add_event(TimerEvent::create(
            ctx.clock + self.interval,
            Box::new(self.clone()),
        ));

        // call tick for every peer
        for peer in 0..ctx.peers.len() {
            tick(ctx, peer);
        }
    }
}
