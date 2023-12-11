use std::io::stdin;

use super::point::Point;

#[derive(Debug)]
pub struct Pod {
    pub position: Point,
    pub speed: Point,
    pub angle: f32,
    pub next_checkpoint_id: usize,
}

impl Pod {
    pub fn init() -> Self {
        let mut raw_input = String::new();
        stdin().read_line(&mut raw_input).unwrap();
        let mut inputs = raw_input.trim().split(' ');
        let position = Point::new(
            inputs.next().unwrap().parse().unwrap(),
            inputs.next().unwrap().parse().unwrap(),
        );
        let speed = Point::new(
            inputs.next().unwrap().parse().unwrap(),
            inputs.next().unwrap().parse().unwrap(),
        );
        let angle = inputs.next().unwrap().parse().unwrap();
        let next_checkpoint_id = inputs.next().unwrap().parse().unwrap();

        Self {
            position,
            speed,
            angle,
            next_checkpoint_id,
        }
    }

    pub fn run(&self, checkpoints: &[Point]) -> String {
        let checkpoint = &checkpoints[self.next_checkpoint_id];
        let target = *checkpoint - self.speed;

        format!("{} {} 100", target.x, target.y)
    }
}
