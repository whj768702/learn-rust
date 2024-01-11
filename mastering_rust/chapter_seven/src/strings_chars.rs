pub fn print_string_chars() {
    let hello = String::from("hello");
    for c in hello.chars() {
        println!("{}", c);
    }
}
