mod pod;
mod point;
pub mod state;

use state::State;

pub fn run(mut state: State) {
    loop {
        state.init_turn();

        dbg!(&state);

        println!("{}", state.pod_1.as_ref().unwrap().run(&state.checkpoints));
        println!("{}", state.pod_2.as_ref().unwrap().run(&state.checkpoints));
    }
}
