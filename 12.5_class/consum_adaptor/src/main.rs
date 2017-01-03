fn main() {
    let hello = vec![1, 2, 3, 4, 5, 6, 7];
    let accept = hello.iter().filter(|&x| *x > 4).map(|&x| x + 1).take(3).collect::<Vec<u32>>();
    println!("{:?}", accept);
}
