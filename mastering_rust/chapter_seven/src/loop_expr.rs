pub fn print_loop_expr() {
    let mut i = 0;
    let counter = loop {
        i += 1;
        if i == 10 {
            break i;
        }
    };

    println!("print_loop_expr: {}", counter);
}
