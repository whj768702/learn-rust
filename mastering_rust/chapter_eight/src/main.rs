fn main() {
    let mut a = 10;
    a = foo();
    println!("Hello, world! {}", a);
}

fn foo() {
    42
}
