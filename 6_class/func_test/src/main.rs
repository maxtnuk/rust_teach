fn sayhello() {
    println!("Hello");
}
fn timehello(x: u32) {
    for _ in 0..x {
        println!("Hello!");
    }
}
fn func_add(x: u32, y: u32) -> u32 {
    x + y
}
fn early_return() -> u32 {
    return 10;
    println!("you can't run this code");
}
fn diverging() -> ! {
    panic!("i don't return the value");
}
fn divide_func(x: u32, y: u32) -> u32 {
    if y == 0 { diverging() } else { x / y }
}
fn main() {
    sayhello();
    timehello(10);
    let y = func_add(10, 11);
    println!("{}", y);
    println!("{}", early_return());
    let f: fn(u32, u32) -> u32 = func_add;
    println!("{}", f(1, 2));
    println!("{}", divide_func(4, 2));
    println!("{}", divide_func(2, 0));
}
