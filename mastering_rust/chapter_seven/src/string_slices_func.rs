fn say_hello(to_whom: &str) {
    println!("Hello, {}!", to_whom);
}

pub fn print_string_slices_func() {
    let string_slice: &'static str = "you";
    let string: String = string_slice.into();
    say_hello(string_slice);
    say_hello(&string);
}

/*
符串切片是一个用途广泛的输入型参数，
不仅适用于实际的字符串切片引用，还适用于 String 引用。所以再强调一遍：如果你需要
将一个字符串传递给你的函数，那么请使用字符串切片&str.

str和String的不同

str: An immutable sequence of characters often used in its borrowed form &str.
It's a view into a sequence of UTF-8 characters that is not necessarily owned
by the code that holds the reference.
It's cheap to pass around as it's just a reference.

String: A heap-allocated, growable, and owned UTF-8 string.
It can be modified, resized, and provides ownership of the data it contains.
It involves memory allocation and thus is more expensive to handle in terms of performance
when compared to &str.

什么时候用哪个
Use &str when you want to read or borrow a string and you don't need ownership,
which is most cases in function parameters.
Use String when you need to own the string data,
for example when you want to store it or mutate it.

fn takes_str(s: &str) {
    println!("{}", s);
}

fn takes_string(s: String) {
    println!("{}", s);
}

fn main() {
    let owned_string = String::from("Hello, World!");
    takes_str(&owned_string); // Borrowing is cheap

    let borrowed_str = "Hello, World!";
    takes_string(borrowed_str.to_string()); // Conversion requires allocation and copying
}
 */
