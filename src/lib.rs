mod pod;
mod point;
pub mod state;

use state::State;

pub fn run(mut state: State) {
    loop {
        state.init_turn();
        state.update();

        dbg!(state.pod_1.as_ref().unwrap().angle_to_checkpoint);
        dbg!(state.pod_2.as_ref().unwrap().angle_to_checkpoint);

        println!("{}", state.pod_1.as_ref().unwrap().run(&state.checkpoints));
        println!("{}", state.pod_2.as_ref().unwrap().run(&state.checkpoints));
    }
}
