#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(x_t: i32, y_t: i32) -> Self {
        Point { x: x_t, y: y_t }
    }
    fn double(&mut self) {
        self.x *= 2;
        self.y *= 2;
    }
    fn len(x_pos: Point, y_pos: Point) -> f64 {
        let x_bet = (x_pos.x - y_pos.x) as f64;
        let y_bet = (x_pos.y - y_pos.y) as f64;
        (x_bet * x_bet + y_bet * y_bet).sqrt()
    }
}
struct Nope;
fn main() {
    let x_point = Point { x: 20, y: 90 };
    let y_point = Point { x: 10, ..x_point };
    println!("{:?}", x_point);
    println!("{:?}", y_point);
    let nope = Nope;
    let mut x_new = Point::new(2, 3);
    let mut y_new = Point::new(10, -2);
    x_new.double();
    y_new.double();
    println!("length is {}", Point::len(x_new, y_new));
}
