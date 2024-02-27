pub mod enum_demo;
pub mod hello_word;
pub mod primitives;
pub mod structures;

use enum_demo::enum_inspect;
use hello_word::hello_world_print;

fn main() {
    println!("enum print starts");
    hello_world_print();
    primitives::primitives_fn();
    primitives::primitives_activity();
    structures::structs_fn();
    enum_inspect();
    println!("enum print ends");
}
