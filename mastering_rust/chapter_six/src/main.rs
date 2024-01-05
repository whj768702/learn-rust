mod result_basics_fixed;
mod using_options_match;

use using_options_match::print_using_options_match;
use using_options_match::print_using_options_unwrap;

use result_basics_fixed::print_result;

fn main() {
    print_using_options_match();
    print_using_options_unwrap();

    print_result();
}
