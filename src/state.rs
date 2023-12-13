use super::pod::Pod;
use super::pod::Strategy;
use super::point::Point;
use std::io::stdin;

#[derive(Debug)]
pub struct State {
    pub laps: u8,
    pub checkpoint_count: u8,
    pub checkpoints: Vec<Point>,
    pub pod_1: Option<Pod>,
    pub pod_2: Option<Pod>,
    pub opponents: [Pod; 2],
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
            opponents: [Pod::default(); 2],
        }
    }

    pub fn init_turn(&mut self) {
        let pod_1 = Pod::init(Strategy::Race);
        let pod_2 = Pod::init(Strategy::Attack);
        let opponent_1 = Pod::init(Strategy::None);
        let opponent_2 = Pod::init(Strategy::None);

        self.pod_1 = Some(pod_1);
        self.pod_2 = Some(pod_2);
        self.opponents[0] = opponent_1;
        self.opponents[1] = opponent_2;
    }

    pub fn update(&mut self) {
        if let Some(pod) = &mut self.pod_1 {
            pod.update(&self.checkpoints);
        }

        if let Some(pod) = &mut self.pod_2 {
            pod.update(&self.checkpoints);
        }
    }
}
