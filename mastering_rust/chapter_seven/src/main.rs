mod loop_expr;
mod string_range_slice;
mod type_alias;

use loop_expr::print_loop_expr;
use string_range_slice::print_string_slice;

fn main() {
    print_loop_expr();
    print_string_slice();
}
