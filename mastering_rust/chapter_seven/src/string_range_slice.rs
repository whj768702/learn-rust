pub fn print_string_slice() {
    let my_str = String::from("Strings are cool.");
    let first_three = &my_str[0..3];
    println!("{:?}", first_three);
}
