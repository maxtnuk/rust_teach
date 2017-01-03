use std::f64;
pub trait ArreaGet {
    fn area(&self) -> f64;
}
#[derive(Clone)]
pub struct square {
    height: u32,
    width: u32,
}
impl square {
    pub fn new(w_t: u32, h_t: u32) -> Self {
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
#[derive(Clone)]
pub struct circle {
    radius: u32,
}
impl circle {
    pub fn new(r_t: u32) -> Self {
        circle { radius: r_t }
    }
}
impl ArreaGet for circle {
    fn area(&self) -> f64 {
        (self.radius * self.radius) as f64 * f64::consts::PI
    }
}
