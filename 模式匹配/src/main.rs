enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;

    let re = match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("North or South");
        }
        _ => println!("West"),
    };
    println!("{:?}", re)
}
