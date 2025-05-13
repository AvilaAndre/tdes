use lazy_static::lazy_static;
use tokio::{
    sync::Mutex,
    time::{Duration, sleep},
};

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Task done!");
    });

    println!("Main function running");
    sleep(Duration::from_secs(2)).await;

    let rounds = 4;

    let mut nodes: Vec<Node> = Vec::new();
    nodes.push(Node::new(3.0, 2.0, 0.0).await);
    nodes.push(Node::new(-3.0, 2.0, 0.0).await);

    for round in 0..rounds {
        println!("Stating round {}", round);
        for node in nodes.iter() {
            println!("node {} ({},{},{})", node.id, node.x, node.y, node.z);
        }
    }
}

lazy_static! {
    static ref NODE_ID: Mutex<u32> = Mutex::new(0);
}

struct Node {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Node {
    async fn new(x: f64, y: f64, z: f64) -> Self {
        let mut id_lock = NODE_ID.lock().await;

        let new_id = *id_lock;
        *id_lock += 1;

        Self {
            id: new_id,
            x,
            y,
            z,
        }
    }
}
