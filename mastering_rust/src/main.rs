mod chapter_one;

use chapter_one::enums::enum_print;
use chapter_one::set_types;
use chapter_one::struct_methods::print_struct_methods;

fn silly_by(a: i32, b: i32) -> i32 {
    let mut result = 0;
    'increase: loop {
        if result == a {
            let mut dec = b;
            'descrement: loop {
                if dec == 0 {
                    break 'increase;
                } else {
                    result -= 1;
                    dec -= 1;
                }
            }
        } else {
            result += 1;
        }
    }
    return result;
}

#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Player {
    name: String,
    iq: u8,
    friends: u8,
    score: u16,
}

fn bump__player_score(mut player: Player, score: u16) {
    player.score += 120;
    println!("Updated player stats:");
    println!("Name: {}", player.name);
    println!("IQ: {}", player.iq);
    println!("Friends: {}", player.friends);
    println!("Score: {}", player.score);
}

fn main() {
    let a = 10;
    let b = 4;
    let result = silly_by(a, b);
    println!("{} minus {} is {}", a, b, result);
    let white = Color(255, 255, 255);
    println!("{:?}", white);

    let red = white.0;
    let green = white.1;
    let blue = white.2;
    println!("{} {} {}", red, green, blue);

    let orange = Color(255, 165, 0);
    let Color(r, g, b) = orange;
    println!("R: {}, G: {}, B: {}", r, g, b);

    let name = "Derek".to_string();
    let player = Player {
        name,
        iq: 171,
        friends: 134,
        score: 1129,
    };
    bump__player_score(player, 120);
    enum_print();

    print_struct_methods();

    set_types::array_print();
    set_types::tuples_print();
    set_types::vec_print();
    set_types::hashmap_print();
    set_types::slices_print();
}
