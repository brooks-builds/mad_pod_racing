use std::{io, ops::Sub};

const TUNE_CARDINAL_BY: f32 = 500.0;
const TUNE_ANGLE_BY: f32 = 350.0;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut checkpoints = Checkpoints::default();
    let mut state = State::ChangingTarget;
    let mut pod = Pod::default();

    loop {
        let mut input_line = String::new();
        let mut opponents = String::new();

        io::stdin().read_line(&mut input_line).unwrap();
        io::stdin().read_line(&mut opponents).unwrap();

        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], f32); // x position of your pod
        let y = parse_input!(inputs[1], f32); // y position of your pod
        let next_checkpoint_x = parse_input!(inputs[2], f32); // x position of the next check point
        let next_checkpoint_y = parse_input!(inputs[3], f32); // y position of the next check point
        let next_checkpoint_distance = parse_input!(inputs[4], f32);
        let next_checkpoint_angle = parse_input!(inputs[5], f32);

        let position = Point::new(x, y);
        let next_checkpoint = Point::new(next_checkpoint_x, next_checkpoint_y);

        pod.calculate_velocity(position);

        match state {
            State::Moving(Target { original, tuned }) => {
                pod.run();

                if original.distance_to(pod.position) < 600.0 {
                    state.change_target();
                }

                let target = if checkpoints.all_mapped
                    && pod.angle.abs() <= 3.0
                    && tuned.unwrap_or(original).distance_to(pod.position) <= 2000.0
                {
                    checkpoints.get_next().clone()
                } else {
                    tuned.unwrap_or(original)
                };

                if next_checkpoint_distance <= pod.velocity * 3.5 {
                    pod.skip_ticks(3);
                }

                println!("{} {} {}", target.x, target.y, pod.get_speed());
            }
            State::ChangingTarget => {
                if !checkpoints.all_mapped {
                    checkpoints.add(next_checkpoint);
                }

                checkpoints.next();

                let target = checkpoints.get();
                state.move_to(target);

                let point = target.tuned.unwrap_or(target.original);
                println!("{} {} 100", point.x, point.y);
            }
        }

        pod.angle = next_checkpoint_angle;
        pod.distance_to_next = next_checkpoint_distance;
    }
}

enum State {
    Moving(Target),
    ChangingTarget,
}

impl State {
    pub fn move_to(&mut self, target: Target) {
        *self = Self::Moving(target);
    }

    pub fn change_target(&mut self) {
        *self = Self::ChangingTarget;
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: Point) -> f32 {
        let difference = *self - other;

        difference.length()
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn cardinal(&self, other: Point, threshold: f32) -> Cardinal {
        let y = (self.y - other.y).abs();
        let x = (self.x - other.x).abs();

        let y = if y > threshold {
            Cardinal::Down
        } else if y < -threshold {
            Cardinal::Up
        } else {
            Cardinal::None
        };

        let x = if x > threshold {
            Cardinal::Right
        } else if x < -threshold {
            Cardinal::Left
        } else {
            Cardinal::None
        };

        Cardinal::combine(x, y)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Default, Debug)]
struct Checkpoints {
    checkpoints: Vec<Point>,
    all_mapped: bool,
    current_checkpoint: usize,
    tuned_checkpoints: Vec<Point>,
}

impl Checkpoints {
    pub fn add(&mut self, checkpoint: Point) {
        if self.have_we_seen_checkpoint(&checkpoint) {
            self.all_mapped = true;
            self.tune_checkpoints();
        } else {
            self.checkpoints.push(checkpoint);
        }
    }

    pub fn next(&mut self) {
        self.current_checkpoint += 1;

        if self.current_checkpoint >= self.checkpoints.len() {
            self.current_checkpoint = 0;
        }
    }

    pub fn get(&self) -> Target {
        let original = self.checkpoints[self.current_checkpoint];
        let tuned = self.tuned_checkpoints.get(self.current_checkpoint).copied();

        Target { original, tuned }
    }

    pub fn get_next(&self) -> &Point {
        self.checkpoints
            .iter()
            .cycle()
            .skip(self.current_checkpoint)
            .next()
            .unwrap()
    }

    fn have_we_seen_checkpoint(&self, checkpoint: &Point) -> bool {
        self.checkpoints.contains(checkpoint)
    }

    fn tune_checkpoints(&mut self) {
        self.tuned_checkpoints = self
            .checkpoints
            .iter()
            .enumerate()
            .map(|(index, &current_checkpoint)| {
                let mut tuned = current_checkpoint;
                let next_checkpoint = self
                    .checkpoints
                    .get(index + 1)
                    .copied()
                    .unwrap_or_else(|| self.checkpoints[0]);
                // this is off sometimes
                let cardinal = next_checkpoint.cardinal(current_checkpoint, 10.0);
                match cardinal {
                    Cardinal::None => (),
                    Cardinal::Up => tuned.y -= TUNE_CARDINAL_BY,
                    Cardinal::UpRight => {
                        tuned.y -= TUNE_ANGLE_BY;
                        tuned.x += TUNE_ANGLE_BY;
                    }
                    Cardinal::Right => tuned.x += TUNE_CARDINAL_BY,
                    Cardinal::DownRight => {
                        tuned.y += TUNE_ANGLE_BY;
                        tuned.x += TUNE_ANGLE_BY;
                    }
                    Cardinal::Down => tuned.y += TUNE_CARDINAL_BY,
                    Cardinal::DownLeft => {
                        tuned.y += TUNE_ANGLE_BY;
                        tuned.x -= TUNE_ANGLE_BY;
                    }
                    Cardinal::Left => tuned.x -= TUNE_CARDINAL_BY,
                    Cardinal::UpLeft => {
                        tuned.y -= TUNE_ANGLE_BY;
                        tuned.x -= TUNE_ANGLE_BY;
                    }
                }

                tuned
            })
            .collect();
    }
}

#[derive(Debug)]
struct Pod {
    position: Point,
    velocity: f32,
    moving: bool,
    angle: f32,
    speed: i32,
    boosts_used: u8,
    boosting: bool,
    distance_to_next: f32,
    ticks_to_skip: u8,
}

impl Pod {
    pub fn calculate_velocity(&mut self, new_position: Point) {
        self.velocity = if self.moving {
            self.position.distance_to(new_position)
        } else {
            self.moving = true;
            0.0
        };

        self.position = new_position;
    }

    pub fn clamp_speed(&mut self) {
        if self.velocity < 100.0 {
            self.speed = 50;
        }
    }

    pub fn boost(&mut self) {
        if self.boosts_used > 0 {
            return;
        }

        self.boosting = true;
        self.boosts_used += 1;
    }

    pub fn get_speed(&self) -> String {
        if self.boosting {
            "BOOST".to_owned()
        } else {
            self.speed.to_string()
        }
    }

    pub fn run(&mut self) {
        self.boosting = false;

        if self.ticks_to_skip > 0 {
            self.ticks_to_skip -= 1;
            self.speed = 0;
            self.clamp_speed();
            return;
        }

        self.speed = 100;

        if self.distance_to_next > 4_000.0 && self.angle.abs() < 0.1 && self.speed == 100 {
            self.boost();
        }
    }

    pub fn skip_ticks(&mut self, ticks: u8) {
        self.ticks_to_skip = ticks;
    }
}

impl Default for Pod {
    fn default() -> Self {
        Self {
            speed: 100,
            distance_to_next: f32::MAX,
            position: Default::default(),
            velocity: Default::default(),
            moving: Default::default(),
            angle: Default::default(),
            boosts_used: Default::default(),
            boosting: Default::default(),
            ticks_to_skip: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Target {
    original: Point,
    tuned: Option<Point>,
}

#[derive(Debug, Copy, Clone)]
enum Cardinal {
    None,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Cardinal {
    pub fn combine(first: Self, second: Self) -> Self {
        match first {
            Cardinal::None => second,
            Cardinal::Up => match second {
                Self::Left => Self::UpLeft,
                Self::Right => Self::UpRight,
                _ => first,
            },
            Cardinal::Right => match second {
                Self::Up => Self::UpRight,
                Self::Down => Self::DownRight,
                _ => first,
            },
            Cardinal::Down => match second {
                Self::Left => Self::DownLeft,
                Self::Right => Self::DownRight,
                _ => first,
            },
            Cardinal::Left => match second {
                Self::Up => Self::UpLeft,
                Self::Down => Self::DownLeft,
                _ => first,
            },
            _ => first,
        }
    }
}
