use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut checkpoints = Checkpoints::default();
    let mut state = State::ChangingTarget;

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
        let _next_checkpoint_distance = parse_input!(inputs[4], f32);
        let _next_checkpoint_angle = parse_input!(inputs[5], f32);

        let position = Point::new(x, y);
        let next_checkpoint = Point::new(next_checkpoint_x, next_checkpoint_y);

        match state {
            State::Moving(target) => {
                eprintln!("moving");
                // are we within 600 of the target?
                if checkpoints.get() != next_checkpoint {
                    state.change_target();
                }
                println!("{} {} 100", target.x, target.y);
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

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Default, Debug)]
struct Checkpoints {
    checkpoints: Vec<Point>,
    all_mapped: bool,
    current_checkpoint: usize,
}

impl Checkpoints {
    pub fn add(&mut self, checkpoint: Point) {
        if self.have_we_seen_checkpoint(&checkpoint) {
            self.all_mapped = true;
            eprintln!("all checkpoints mapped");
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

    fn have_we_seen_checkpoint(&self, checkpoint: &Point) -> bool {
        self.checkpoints.contains(checkpoint)
    }
}
