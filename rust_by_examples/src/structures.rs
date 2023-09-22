// 单元结构体 unit structs
struct Unit;

struct Point {
    x: f32,
    y: f32,
}

// 元组结构体 tuple structs
struct Pair(i32, f32);

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub fn structs_fn() {
    let point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 语法 .. 指定未显式设置的其余字段应与给定实例中的字段具有相同的值。
    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };
    println!(
        "Rectangle: ({}, {}), ({}, {})",
        rectangle.top_left.x,
        rectangle.top_left.y,
        rectangle.bottom_right.x,
        rectangle.bottom_right.y
    );

    let rectangle1 = Rectangle {
        top_left: Point { x: 4.0, y: 5.0 },
        bottom_right: Point { x: 8.0, y: 10.0 },
    };
    let area = rect_area(rectangle1);
    println!("area: {}", area);

    let square = square(&point, 5.0);
    println!(
        "square: ({}, {}), ({}, {})",
        square.top_left.x, square.top_left.y, square.bottom_right.x, square.bottom_right.y
    );
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: Point {
            x: right_edge,
            y: bottom_edge,
        },
    } = rectangle;

    let width = right_edge - left_edge;
    let height = bottom_edge - top_edge;

    return width * height;
}

fn square(point: &Point, size: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: point.x,
            y: point.y,
        },
        bottom_right: Point {
            x: point.x + size,
            y: point.y + size,
        },
    }
}
