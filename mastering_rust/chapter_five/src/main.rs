mod linked_list;
use linked_list::print_linked_list;

#[derive(Debug)]
struct Number<'a> {
    num: &'a u8,
}
impl<'a> Number<'a> {
    fn get_num(&self) -> &'a u8 {
        self.num
    }
    fn set_num(&mut self, new_number: &'a u8) {
        self.num = new_number
    }
}

struct Character {
    name: String,
}

impl Drop for Character {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn main() {
    let a = 10;
    let mut num = Number { num: &a };
    num.set_num(&23);
    println!("Number: {}", num.get_num());

    let steve = Character {
        name: "Steve".into(),
    };
    let john = Character {
        name: "John".into(),
    };

    println!("{} {}", steve.name, john.name);

    print_linked_list();
}
