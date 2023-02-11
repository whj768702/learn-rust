use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {
        name: String::from("Asparagus"),
    };
    println!("I'm growing {:?}!", plant.name);
}
