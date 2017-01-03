use std::f64;
trait ArreaGet {
    fn area(&self) -> f64;
}
struct square {
    height: u32,
    width: u32,
}
impl square {
    fn new(w_t: u32, h_t: u32) -> Self {
        square {
            height: h_t,
            width: w_t,
        }
    }
}
impl ArreaGet for square {
    fn area(&self) -> f64 {
        (self.height * self.width) as f64
    }
}
struct circle {
    radius: u32,
}
impl circle {
    fn new(r_t: u32) -> Self {
        circle { radius: r_t }
    }
}
impl ArreaGet for circle {
    fn area(&self) -> f64 {
        (self.radius * self.radius) as f64 * f64::consts::PI
    }
}
fn main() {
    let x_s = square::new(20, 18);
    let x_c = circle::new(10);
    println!("{}", x_s.area());
    println!("{}", x_c.area());
}
