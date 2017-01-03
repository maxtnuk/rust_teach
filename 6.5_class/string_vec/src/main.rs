fn main() {
    let hell = "hell";
    let result = &hell[0..1];
    println!("{}", result);
    let string_t = hell.to_string();
    println!("{}", string_t);
    println!("{}", result.to_string() + "eaven");
    let mut x = vec![1, 2, 3, 4];
    let index = 0usize;
    println!("{}", x[index]);
    x.push(5);
    println!("{:?}", x);
    for i in x.iter() {
        // x.push(10);
        println!("{}", i);
    }
}
