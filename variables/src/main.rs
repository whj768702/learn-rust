fn main() {
    let mut x = 5;
    println!("the value of x is: {}", x);
    x = 6;
    println!("the value of x is: {}", x);
    let _y = 10; // 未使用变量加下划线开头，告诉rust编译器不用告警

    // 变量解构
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);

    // 常量:使用const,并且值得类型必须标注
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);
}
