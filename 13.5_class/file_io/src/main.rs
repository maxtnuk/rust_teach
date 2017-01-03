use std::io;
use std::fmt;
extern crate rand;
fn control_err() -> ! {
    panic!("fail to run");
}
struct file_content {
    content: Vec<u8>,
}
impl fmt::Display for file_content {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for (index, element) in self.content.iter().enumerate() {
            result.push_str(format!("{:x}", element).as_str());
            if index % 8 == 7 {
                result.push_str("\n");
            }
        }
        write!(f, "{}", result)
    }
}
fn main() {
    {
        let mut x_string = String::new();
        io::stdin().read_line(&mut x_string).expect("fail to read");
        println!("read some string {}", x_string);
    }
    {
        use std::io::Read;
        let mut x_vec: Vec<u8> = Vec::new();
        io::stdin().read_to_end(&mut x_vec).expect("fail to read");
        println!("read some array {:?}", x_vec);
    }
    {
        use std::io::{prelude, Read};
        use std::fs::File;
        let mut fs_create = File::create("hello.txt").expect("fail to crate");

        let mut fs = match File::open("hello.txt") {
            Ok(file) => file,
            Err(_) => control_err(),
        };
        let mut content: Vec<u8> = Vec::new();
        match fs.read(&mut content) {
            Ok(_) => {
                println!("read complete");
            }
            Err(_) => {
                control_err();
            }
        }
        let i_have = file_content { content: content };
        println!("here is the file content\n{}", i_have);
    }
}
