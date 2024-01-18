trait Circular {
    const PI: f64 = 3.14;
    fn area(&self) -> f64;
}

struct Circle {
    rad: f64,
}
impl Circular for Circle {
    fn area(&self) -> f64 {
        Circle::PI * self.rad * self.rad
    }
}

pub fn print_trait_constants() {
    let c_one = Circle { rad: 4.2 };
    let c_two = Circle { rad: 75.2 };
    println!("Circle one面积是: {}", c_one.area());
    println!("Circle two面积是: {}", c_two.area());
}
