mod enums;
mod set_types;
mod struct_methods;
mod word_counter;

use enums::enum_print;
use set_types::array_print;
use set_types::hashmap_print;
use set_types::slices_print;
use set_types::tuples_print;
use set_types::vec_print;
use struct_methods::print_struct_methods;
use word_counter::run_word_counter;

fn main() {
    println!("Hello, world!");
    enum_print();

    array_print();
    tuples_print();
    vec_print();
    hashmap_print();
    slices_print();

    print_struct_methods();

    run_word_counter();
}
