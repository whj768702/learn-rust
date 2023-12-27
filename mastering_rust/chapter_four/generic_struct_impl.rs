pub struct Container<T> {
    item: T,
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }
}

pub fn print_generic() {
    let c = Container::new("hello");
    println!("{}", c.item);
}
