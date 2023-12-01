use std::io;

const CHECKPOINT_RADIUS: f32 = 600.0;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Default, Debug)]
struct GameState {
    used_boost: bool,
    x: f32,
    y: f32,
    next_checkpoint_x: f32,
    next_checkpoint_y: f32,
    next_checkpoint_distance: f32,
    next_checkpoint_angle: f32,
    distance_weight: f32,
    break_point_distance: f32,
    angular_weight: f32,
    max_thrust: f32,
    speed: f32,
    min_distance_to_boost: f32,
    checkpoints: Vec<Checkpoint>,
    current_checkpoint_index: Option<usize>,
}

impl GameState {
    pub fn new() -> Self {
        let distance_weight = 0.2;
        let break_point_distance = 600.0;
        let angular_weight = 1.0;
        let max_thrust = 100.0;
        let min_distance_to_boost = 5000.0;

        Self {
            distance_weight,
            break_point_distance,
            angular_weight,
            max_thrust,
            min_distance_to_boost,
            ..Default::default()
        }
    }

    pub fn update_state(&mut self) {
        let mut input_line = String::new();
        let mut opponents = String::new();

        io::stdin().read_line(&mut input_line).unwrap();
        io::stdin().read_line(&mut opponents).unwrap();

        let inputs = input_line.split(" ").collect::<Vec<_>>();

        self.x = parse_input!(inputs[0], f32); // x position of your pod
        self.y = parse_input!(inputs[1], f32); // y position of your pod
        self.next_checkpoint_x = parse_input!(inputs[2], f32); // x position of the next check point
        self.next_checkpoint_y = parse_input!(inputs[3], f32); // y position of the next check point
        self.next_checkpoint_distance = parse_input!(inputs[4], f32);
        self.next_checkpoint_angle = parse_input!(inputs[5], f32);

        // if we are at the next checkpoint, set it as our destination
        let next_checkpoint = Checkpoint::new(self.next_checkpoint_x, self.next_checkpoint_y);

        if !self.have_we_seen_next_checkpoint(&next_checkpoint) {
            self.checkpoints.push(next_checkpoint);
        }

        if self.current_checkpoint_index.is_none() {
            self.current_checkpoint_index = Some(0);
        }

        if self.are_we_at_next_checkpoint() {
            dbg!("got to next checkpoint!");
            let checkpoints_length = self.checkpoints.len();
            self.current_checkpoint_index = self.current_checkpoint_index.map(|mut index| {
                index += 1;

                if index >= checkpoints_length {
                    index = 0;
                }

                index
            });
        }
    }

    pub fn set_speed(&mut self) {
        let raw_speed = self.distance_weight
            * (self.next_checkpoint_distance / self.break_point_distance).min(1.0)
            + self.angular_weight * (1.0 - self.next_checkpoint_angle.abs() / 90.0);
        let normalized_speed = raw_speed * (1.0 / (self.angular_weight + self.distance_weight));
        let speed = normalized_speed * self.max_thrust;

        self.speed = speed.clamp(0.0, 100.0);
    }

    pub fn run(&mut self) -> String {
        let speed = if self.should_boost() {
            self.used_boost = true;
            "BOOST".to_owned()
        } else {
            (self.speed as i32).to_string()
        };

        let index = self.current_checkpoint_index.expect("missing index");
        let checkpoint = self
            .checkpoints
            .get(index)
            .map(Clone::clone)
            .expect("missing checkpoint");

        format!("{} {} {speed}", checkpoint.x, checkpoint.y)
    }

    pub fn should_boost(&self) -> bool {
        !self.used_boost
            && self.next_checkpoint_angle == 0.0
            && self.next_checkpoint_distance > self.min_distance_to_boost
    }

    pub fn are_we_at_next_checkpoint(&self) -> bool {
        let index = self
            .current_checkpoint_index
            .expect("no index when checking if we are at next checkpoint");
        let checkpoint = &self.checkpoints[index];
        let delta_x = (checkpoint.x - self.x).abs();
        let delta_y = (checkpoint.y - self.y).abs();
        let distance = (delta_x.powi(2) + delta_y.powi(2)).sqrt();

        distance <= CHECKPOINT_RADIUS - self.break_point_distance
    }

    fn have_we_seen_next_checkpoint(&self, next: &Checkpoint) -> bool {
        for checkpoint in self.checkpoints.iter() {
            if next == checkpoint {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, PartialEq, Default, Clone)]
struct Checkpoint {
    x: f32,
    y: f32,
    radius: f32,
}

impl Checkpoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            radius: CHECKPOINT_RADIUS,
        }
    }
}

/**
 * This code automatically collects game data in an infinite loop.
 * It uses the standard input to place data into the game variables such as x and y.
 * YOU DO NOT NEED TO MODIFY THE INITIALIZATION OF THE GAME VARIABLES.
 **/
fn main() {
    let mut game_state = GameState::new();

    // game loop
    loop {
        game_state.update_state();
        game_state.set_speed();

        dbg!(
            &game_state.current_checkpoint_index,
            &game_state.checkpoints
        );

        // Write an action using println!("message...");
        println!("{}", game_state.run());
        // To debug: eprintln!("Debug message...");

        // Edit this line to output the target position
        // and thrust (0 <= thrust <= 100)
        // i.e.: "x y thrust"
        // println!("{} {} 100", next_checkpoint_x, next_checkpoint_y);
        // dbg!(x, y, next_checkpoint_x, next_checkpoint_y, next_checkpoint_distance, next_checkpoint_angle);
    }
}
