#[warn(unused_variables)]
fn main() {
    let mut x = 10;
    println!("{}", x + 1);
    println!("{}", x - 2);
    println!("{}", x * 2);
    println!("{}", x / 2);
    println!("{}", x % 3);
    println!("{}", {
        x += 1;
        x
    });
    println!("{}", {
        x -= 1;
        x
    });
    println!("{}", {
        x /= 2;
        x
    });
    println!("{}", {
        x *= 2;
        x
    });
    let y = 10;
    println!("{}", x == y);
    println!("{}", x != y);
    println!("{}", {
        let y = 9;
        y <= x
    });
    println!("{}", {
        let y = 9;
        y > x
    });
    println!("{}", x >> 1);
}
