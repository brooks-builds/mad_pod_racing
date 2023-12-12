use std::io::stdin;

use super::point::Point;

#[derive(Debug)]
pub struct Pod {
    pub position: Point,
    pub speed: Point,
    pub angle: f32,
    pub next_checkpoint_id: usize,
    pub angle_to_checkpoint: f32,
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
        let angle_to_checkpoint = 0.0; // we always start the race facing the first checkpoint

        Self {
            position,
            speed,
            angle,
            next_checkpoint_id,
            angle_to_checkpoint,
        }
    }

    pub fn run(&self, checkpoints: &[Point]) -> String {
        let checkpoint = &checkpoints[self.next_checkpoint_id];
        let target = *checkpoint - self.speed;
        let speed = if self.angle_to_checkpoint > 75.0 {
            85
        } else {
            100
        };

        format!("{} {} {speed}", target.x, target.y)
    }

    pub fn update(&mut self, checkpoints: &[Point]) {
        let checkpoint = &checkpoints[self.next_checkpoint_id];

        self.calculate_angle_to_checkpoint(*checkpoint);
    }

    pub fn calculate_angle_to_checkpoint(&mut self, checkpoint: Point) {
        let distance = checkpoint - self.position;
        let angle_to_x = distance.y.atan2(distance.x).to_degrees();
        let angle = angle_to_x - self.angle;
        let angle = ((angle.abs() + 180.0) % 360.0) - 180.0;

        self.angle_to_checkpoint = angle.abs();
    }
}
