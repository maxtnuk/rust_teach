fn main() {
    let x = 10;
    let y = x;
    println!("{}", y);
    println!("{}", x);
    let vec_x = vec![1, 2, 3];
    let vec_clone = vec_x.clone();
    let vec_y = vec_x;
    println!("{:?}", vec_y);
    println!("{:?}", vec_clone);
    // println!("{:?}", vec_x);
}
