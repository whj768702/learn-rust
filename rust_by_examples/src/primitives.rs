use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)?;
        write!(f, "\n({}, {})", self.2, self.3)
    }
}

pub fn primitives_fn() {
    // 整数8、16、32、64、128、size, 有符号(i)，无符号(u)
    // 浮点数32、64, (f)
    // char, 'a', 'b'等
    // bool true, false
    // unit type (), ()

    // 复合类型
    // 数组, [1, 2, 3]
    // 元组, (1, 2, true)
    println!("1 + 2 = {}", 1u32 + 2); // 无符号32位整数

    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("reversed pair is {:?}", reverse_tuples(pair));
}

pub fn reverse_tuples(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    return (boolean, integer);
}

fn transpose(matrix: Matrix) -> Matrix {
    let a = matrix.0;
    let b = matrix.1;
    let c = matrix.2;
    let d = matrix.3;

    return Matrix(a, c, b, d);
}

pub fn primitives_activity() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("self format Matrix:\n{}", matrix);
    println!("Matrix:\n{:?}", matrix);
    println!("transpose:\n{}", transpose(matrix));
}
