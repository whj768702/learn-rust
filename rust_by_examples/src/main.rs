pub mod hello_word;
pub mod primitives;
pub mod structures;

use hello_word::hello_world_print;

fn main() {
    hello_world_print();
    primitives::primitives_fn();
    primitives::primitives_activity();
    structures::structs_fn();
}
