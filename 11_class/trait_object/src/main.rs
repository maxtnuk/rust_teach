extern crate trait_object;
use trait_object::*;
use std::boxed::Box;
fn compare<T, K>(x: T, y: K)
    where T: ArreaGet,
          K: ArreaGet
{
    if x.area() > y.area() {
        println!("first is bigger than second");
    } else if x.area() < y.area() {
        println!("second is bigger than first");
    } else {
        println!("they are same");
    }
}
fn compare_dynamic(x: &ArreaGet, y: &ArreaGet) {
    if x.area() > y.area() {
        println!("first is bigger than second");
    } else if x.area() < y.area() {
        println!("second is bigger than first");
    } else {
        println!("they are same");
    }
}
fn compare_box(x: Box<ArreaGet>, y: Box<ArreaGet>) {
    if x.area() > y.area() {
        println!("first is bigger than second");
    } else if x.area() < y.area() {
        println!("second is bigger than first");
    } else {
        println!("they are same");
    }
}
fn main() {
    let x = circle::new(20);
    let y = square::new(12, 13);
    compare(y.clone(), x.clone());
    compare_dynamic(&x, &y);
    compare_box(Box::new(x), Box::new(y));
}
