use std::{io, ops::Sub};

const DECELERATION_DISTANCE: f32 = 2400.0;
const DECELERATION_SPEED: i32 = 25;
const HIGH_ANGLE_SPEED: i32 = 50;
const HIGH_ANGLE: f32 = 45.0;
const MINIMUM_VELOCITY: f32 = 400.0;

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
            State::Moving(target) => {
                eprintln!("moving, our velocity is {}", pod.velocity);
                let mut speed = 100;

                // do we need to turn a lot to point towards the next target?
                if next_checkpoint_angle >= HIGH_ANGLE {
                    speed = HIGH_ANGLE_SPEED;
                }
                // are we close to the target?
                if next_checkpoint_distance <= DECELERATION_DISTANCE {
                    speed = DECELERATION_SPEED;
                }

                if pod.velocity <= MINIMUM_VELOCITY {
                    speed = 100;
                }

                let speed =
                    if checkpoints.is_boost_time() && next_checkpoint_angle < 5.0 && speed == 100 {
                        "BOOST".to_owned()
                    } else {
                        speed.to_string()
                    };

                // are we within 600 of the target?
                if checkpoints.get() != next_checkpoint {
                    state.change_target();
                }
                println!("{} {} {speed}", target.x, target.y);
            }
            State::ChangingTarget => {
                eprintln!("changing target");
                // choose next target

                checkpoints.add(next_checkpoint);
                checkpoints.next();
                let point = checkpoints.get();
                state.move_to(point);
                // now move
                println!("{} {} 100", point.x, point.y);
            }
        }
    }
}

enum State {
    Moving(Point),
    ChangingTarget,
}

impl State {
    pub fn move_to(&mut self, point: Point) {
        *self = Self::Moving(point);
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
    boost_on: Option<usize>,
}

impl Checkpoints {
    pub fn add(&mut self, checkpoint: Point) {
        if self.have_we_seen_checkpoint(&checkpoint) {
            self.all_mapped = true;
            eprintln!("all checkpoints mapped");
            self.calculate_boost_checkpoint();
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

    pub fn get(&self) -> Point {
        self.checkpoints[self.current_checkpoint]
    }

    pub fn is_boost_time(&self) -> bool {
        let current_index = self.current_checkpoint;
        self.boost_on
            .is_some_and(move |index| index == current_index)
    }

    fn have_we_seen_checkpoint(&self, checkpoint: &Point) -> bool {
        self.checkpoints.contains(checkpoint)
    }

    fn calculate_boost_checkpoint(&mut self) {
        let checkpoints = self.checkpoints.clone();
        let distances = self
            .checkpoints
            .iter()
            .zip(checkpoints.iter().skip(1))
            .map(|(one, two)| one.distance_to(*two))
            .collect::<Vec<f32>>();
        let mut longest_distance = None;
        let mut longest_distance_index = None;

        for (index, distance) in distances.iter().enumerate() {
            match longest_distance {
                Some(unwrapped_longest_distance) => {
                    if distance > unwrapped_longest_distance {
                        longest_distance = Some(distance);
                        longest_distance_index = Some(index);
                    }
                }
                None => {
                    longest_distance = Some(distance);
                    longest_distance_index = Some(index);
                }
            }
        }

        self.boost_on = longest_distance_index;
    }
}

#[derive(Default, Debug)]
struct Pod {
    position: Point,
    velocity: f32,
    moving: bool,
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
}
