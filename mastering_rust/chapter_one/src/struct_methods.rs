// impl关键字用于实现某个类的方法或者关联函数。你可以使用它来定义或实现一个特定类型的方法，或者实现一个trait
struct Player {
    name: String,
    friends: u8,
}

impl Player {
    fn with_name(name: &str) -> Player {
        Player {
            name: name.to_string(),
            friends: 100,
        }
    }
    fn get_friends(&self) -> u8 {
        self.friends
    }
    fn set_friends(&mut self, count: u8) {
        self.friends = count;
    }
}

pub fn print_struct_methods() {
    let mut player = Player::with_name("Derek");
    player.set_friends(23);
    println!("{}'s friends count: {}", player.name, player.get_friends());
}
