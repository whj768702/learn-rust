mod custom_iterator;
mod loop_expr;
mod string_range_slice;
mod string_slices_func;
mod strings_chars;
mod trait_constants;
mod type_alias;

use custom_iterator::print_primies;
use loop_expr::print_loop_expr;
use string_range_slice::print_string_slice;
use string_slices_func::print_string_slices_func;
use strings_chars::print_string_chars;
use trait_constants::print_trait_constants;

fn main() {
    print_loop_expr();
    print_string_slice();
    print_string_chars();
    print_string_slices_func();
    print_primies();
    print_trait_constants();
}
