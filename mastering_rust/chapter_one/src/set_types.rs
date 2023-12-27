use std::collections::HashMap;

// 数组的定义方式，[T; N]，其中T是数组元素类型，N是数组长度
pub fn array_print() {
    let numbers: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let floats = [0.1f64, 0.2, 0.3];

    println!("Number: {}", numbers[5]);
    println!("Float: {}", floats[2]);
}

pub fn tuples_print() {
    let num_and_str: (u8, &str) = (10, "hello");
    println!("{:?}", num_and_str);

    let (num, string) = num_and_str;
    println!("From tuple: Number: {}, String: {}", num, string);
}

// vec是堆上分配的。可以使用构造函数 Vec::new，也可以使用宏 vec![]创建
pub fn vec_print() {
    let mut numbers_vec: Vec<u8> = Vec::new();
    numbers_vec.push(1);
    numbers_vec.push(2);

    let mut vec_with_macro = vec![1];
    vec_with_macro.push(2);

    let message = if numbers_vec == vec_with_macro {
        "They are equal"
    } else {
        "Nah! They look different to me"
    };

    println!("{} {:?} {:?}", message, numbers_vec, vec_with_macro);
}

pub fn hashmap_print() {
    let mut fruits = HashMap::new();
    fruits.insert("apple", 3);
    fruits.insert("mango", 6);
    fruits.insert("orange", 2);
    fruits.insert("avocado", 7);

    for (k, v) in &fruits {
        println!("I got {} {}", v, k);
    }

    fruits.remove("orange");
    let old_avocado = fruits["avocado"];
    fruits.insert("avocado", old_avocado + 5);
    println!("\n I now have {} avocados", fruits["avocado"]);
}

pub fn slices_print() {
    let mut numbers: [u8; 4] = [1, 2, 3, 4];
    {
        let all: &[u8] = &numbers[..];
        println!("All of them: {:?}", all);
    }
    {
        let first_two: &mut [u8] = &mut numbers[0..2];
        first_two[0] = 100;
        first_two[1] = 99;
    }
    println!("Look ma! I can modify through slices: {:?}", numbers);
}
