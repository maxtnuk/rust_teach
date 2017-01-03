const K: u32 = 10;
static mut N: u32 = 10;
fn main() {
    println!("{}", K);
    unsafe {
        N += 1;
        println!("{}", N);
    }
}
