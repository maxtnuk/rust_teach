struct Point {
    x: u32,
    y: u32,
}
fn main() {
    let x = 12;
    match x {
        1 => println!("one"),
        2...9 => println!("many!!"),
        10 | 13 => println!("ten or thirteen"),
        x @ _ => println!("else number {}", x),
    }
    let y = Point { x: 10, y: 11 };
    match y {
        Point { x: x_t, y: y_t } => {
            println!("{} {}", x_t, y_t);
        }
    }
}
