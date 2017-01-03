fn give_closure() -> Box<Fn(u32) -> u32> {
    Box::new(move |x: u32| -> u32 { x + 2 })
}
fn main() {
    let mut x = 3;
    {
        let mut addnum = move |number: u32| {
            x += number;
            println!("{}", x);
        };
        addnum(10);
    }
    println!("{}", x);
    let result = give_closure();
    println!("{}", result(10));
}
