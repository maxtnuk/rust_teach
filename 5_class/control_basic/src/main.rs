use std::io;
fn main() {
    loop {
        println!("insert the number between 1...10");
        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("fail to read");
        let x: u32 = match x.trim().parse() {
            Ok(result) => result,
            Err(_) => {
                println!("fail to parse");
                continue;
            }
        };
        if x == 0 {
            println!("i want to go outside");
            break;
        }
        let mut find_it = false;
        for bet in 1..11 {
            if bet == x {
                println!("you put this number {} ", x);
                find_it = true;
                break;
            } else {
                println!("not yet");
            }
        }
        if find_it {
            println!("let's say hello {} times", x);
        } else {
            println!("i don't want say hello");
        }
        let mut x = x;
        while x != 0 && find_it {
            println!("hello");
            x -= 1;
        }
    }
}
