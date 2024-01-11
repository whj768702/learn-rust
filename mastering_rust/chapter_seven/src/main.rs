mod loop_expr;
mod string_range_slice;
mod strings_chars;
mod type_alias;

use loop_expr::print_loop_expr;
use string_range_slice::print_string_slice;
use strings_chars::print_string_chars;

fn main() {
    print_loop_expr();
    print_string_slice();
    print_string_chars();
}
