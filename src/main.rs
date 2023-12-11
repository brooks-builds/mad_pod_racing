extern crate mad_pod_racing;

use mad_pod_racing::{run, state::State};

fn main() {
    let state = State::init();

    run(state);
}
