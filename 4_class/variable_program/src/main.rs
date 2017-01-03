#[warn(unused_variables)]//ignore the unused variable
fn main() {
    let x = 1;//u32
    let x_8: u8 = 128;
    let f_d_32: f64 = 10.0;//f64
    let f_s_32 = 10f32;
    let mut mut_x = 99;//mutable
    mut_x = 43;
    {
        println!("{}", mut_x);//outer variable show
        let mut_x = 11;
        println!("{}", mut_x);//inner variable show
        // mut_x = 1;
    }
    println!("{}", mut_x);//outer variable show
    let mut_x = 40;
    println!("{}", mut_x);//new variable show
    // mut_x = 45;
}
