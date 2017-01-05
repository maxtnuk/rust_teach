use std::ops::{Add, Mul, Sub};
struct Point<T> {
    x: T,
    y: T,
}
trait PointFunc {
    type OB;
    type Return;
    fn len_sqaure(first: Self::OB, second: Self::OB) -> Self::Return;
}
impl<T> PointFunc for Point<T>
    where T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy
{
    type OB = Self;
    type Return = T;
    fn len_sqaure(first: Self::OB, second: Self::OB) -> Self::Return {
        let x_bt = first.x - second.x;
        let y_bt = first.y - second.y;
        x_bt * x_bt + y_bt * y_bt
    }
}
fn main() {
    {
        let first = Point { x: 12, y: 11 };
        let second = Point { x: 22, y: 34 };
        println!("{}", Point::len_sqaure(first, second));
    }
    {
        let first = Point {
            x: 11.2_f64,
            y: 1.2_f64,
        };
        let second = Point {
            x: 7.09_f64,
            y: 17.01_f64,
        };
        println!("{}", Point::len_sqaure(first, second));
    }
}
