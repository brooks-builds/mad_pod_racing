use super::pod::Pod;
use super::point::Point;
use std::io::stdin;

#[derive(Debug)]
pub struct State {
    pub laps: u8,
    pub checkpoint_count: u8,
    pub checkpoints: Vec<Point>,
    pub pod_1: Option<Pod>,
    pub pod_2: Option<Pod>,
    pub opponent_1: Option<Pod>,
    pub opponent_2: Option<Pod>,
}

impl State {
    pub fn init() -> Self {
        let mut laps = String::new();
        let mut checkpoint_count = String::new();

        stdin().read_line(&mut laps).unwrap();
        stdin().read_line(&mut checkpoint_count).unwrap();

        let checkpoint_count = checkpoint_count.trim().parse().unwrap();
        let mut checkpoints = vec![];

        for _checkpoint_index in 0..checkpoint_count {
            let mut checkpoint = String::new();
            stdin().read_line(&mut checkpoint).unwrap();
            let checkpoint = Point::from(checkpoint);
            checkpoints.push(checkpoint);
        }

        Self {
            laps: laps.trim().parse().unwrap(),
            checkpoint_count,
            checkpoints,
            pod_1: None,
            pod_2: None,
            opponent_1: None,
            opponent_2: None,
        }
    }

    pub fn init_turn(&mut self) {
        let pod_1 = Pod::init();
        let pod_2 = Pod::init();
        let opponent_1 = Pod::init();
        let opponent_2 = Pod::init();

        self.pod_1 = Some(pod_1);
        self.pod_2 = Some(pod_2);
        self.opponent_1 = Some(opponent_1);
        self.opponent_2 = Some(opponent_2);
    }
}
