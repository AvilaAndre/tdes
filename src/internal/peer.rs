pub struct Peer {
    pub id: Option<usize>,
    pub pos: (f64, f64, f64),
}

impl Peer {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            id: None,
            pos: (x, y, z),
        }
    }
}

