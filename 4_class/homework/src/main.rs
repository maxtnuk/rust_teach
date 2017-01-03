fn main() {
    let x = 10;
    {
        println!("{}", x);
        let x = 9;
        println!("{}", x);
    }
}
