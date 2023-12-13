mod pod;
mod point;
pub mod state;

use state::State;

pub fn run(mut state: State) {
    loop {
        state.init_turn();
        state.update();

        let closest_opponent = state.pod_1.unwrap().closest_to_me(&state.opponents);

        println!(
            "{}",
            state
                .pod_1
                .as_ref()
                .unwrap()
                .run(&state.checkpoints, None)
                .unwrap()
        );
        println!(
            "{}",
            state
                .pod_2
                .as_ref()
                .unwrap()
                .run(&state.checkpoints, Some(closest_opponent))
                .unwrap()
        );
    }
}
