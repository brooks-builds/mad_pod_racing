use std::{io, ops::Sub};

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
                pod.run();

                if checkpoints.get() != next_checkpoint {
                    state.change_target();
                }

                let target = if checkpoints.all_mapped
                    && next_checkpoint_angle.abs() <= 3.0
                    && next_checkpoint_distance <= 2000.0
                {
                    checkpoints.get_next().clone()
                } else {
                    target
                };

                if next_checkpoint_distance <= pod.velocity * 3.5 {
                    pod.skip_ticks(3);
                }

                println!("{} {} {}", target.x, target.y, pod.get_speed());
            }
            State::ChangingTarget => {
                checkpoints.add(next_checkpoint);
                checkpoints.next();
                let point = checkpoints.get();
                state.move_to(point);

                println!("{} {} 100", point.x, point.y);
            }
        }

        pod.angle = next_checkpoint_angle;
        pod.distance_to_next = next_checkpoint_distance;
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

    pub fn get_next(&self) -> &Point {
        dbg!("getting next:");
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
        if self.velocity < 200.0 {
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

        if self.distance_to_next > 10_000.0 && self.angle.abs() < 0.1 && self.speed == 100 {
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
