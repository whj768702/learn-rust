use std::collections::HashMap;

// match语句匹配返回的结果Option<T>
pub fn print_using_options_match() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);

    let incremented_value = match map.get("one") {
        Some(val) => val + 1,
        None => 0,
    };
    println!("{}", incremented_value);
}

// unwrap解压Option
// 在 Rust 中，unwrap() 和 expect() 都是用来处理 Option 和 Result 类型的方法。
// 不过，如果直接使用它们，当 Option 是 None 或者 Result 是 Err 时，它们都会导致程序 panic。
// 为了避免这种错误，你应该在调用 unwrap() 或 expect() 之前检查值是否存在。
// 如果你确信 Option 或 Result 中绝对有值，你可以使用 unwrap()。
// 但这种情况很少见，通常你应该避免这么做，因为这会增加程序在运行时崩溃的风险。
// expect() 方法与 unwrap() 类似，但它允许你指定一个错误消息，当 Option 为 None 或 Result 为 Err 时，这个错误消息会被输出。
// 它提供了更多的上下文信息，有助于调试。
pub fn print_using_options_unwrap() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);

    let incremented_value = map.get("one").unwrap() + 1;
    println!("{}", incremented_value);
}
