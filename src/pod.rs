use super::point::Point;
use std::io::stdin;

const TURNING_SPEED: i32 = 85;
const MAX_SPEED: i32 = 100;

#[derive(Debug, Default, Clone, Copy)]
pub struct Pod {
    pub position: Point,
    pub speed: Point,
    pub angle: f32,
    pub next_checkpoint_id: usize,
    pub angle_to_checkpoint: f32,
    pub strategy: Strategy,
}

impl Pod {
    pub fn init(strategy: Strategy) -> Self {
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
            strategy,
        }
    }

    pub fn run(&self, checkpoints: &[Point], opponent: Option<Pod>) -> Option<String> {
        match self.strategy {
            Strategy::Race => Some(self.run_race(checkpoints)),
            Strategy::Attack => Some(self.run_attack(opponent.unwrap())),
            _ => None,
        }
    }

    fn run_race(&self, checkpoints: &[Point]) -> String {
        let checkpoint = &checkpoints[self.next_checkpoint_id];
        let target = *checkpoint - self.speed;
        let speed = if self.angle_to_checkpoint > 75.0 {
            85
        } else {
            100
        };

        format!("{} {} {speed}", target.x, target.y)
    }

    fn run_attack(&self, target: Pod) -> String {
        let target_point = target.position + target.speed;
        let target_point = target_point - self.speed;
        let angle_to_target = self.calculate_angle_to(target_point);
        let speed = if angle_to_target > 75.0 {
            TURNING_SPEED
        } else {
            MAX_SPEED
        };
        let Point { x, y } = target_point;

        format!("{x} {y} {speed}")
    }

    pub fn update(&mut self, checkpoints: &[Point]) {
        let checkpoint = &checkpoints[self.next_checkpoint_id];

        self.angle_to_checkpoint = self.calculate_angle_to(*checkpoint);
    }

    pub fn calculate_angle_to(&self, other: Point) -> f32 {
        let distance = other - self.position;
        let angle_to_x = distance.y.atan2(distance.x).to_degrees();
        let angle = angle_to_x - self.angle;
        let angle = ((angle.abs() + 180.0) % 360.0) - 180.0;

        angle.abs()
    }

    pub fn closest_to_me(&self, others: &[Pod]) -> Pod {
        let distance = self.position.distance_to(&others[0].position);

        if self.position.distance_to(&others[1].position) < distance {
            others[1]
        } else {
            others[0]
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Strategy {
    Attack,
    Race,
    #[default]
    None,
}
