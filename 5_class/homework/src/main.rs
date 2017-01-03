fn main() {
    let mut first = 0;
    let mut second = 1;
    print!("{} {}", first, second);
    for _ in 0..8 {
        let third = first + second;
        print!(" {}", third);
        first = second;
        second = third;
    }
    println!("");
}
