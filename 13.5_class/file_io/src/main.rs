extern crate rand;
use rand::Rng;
use std::io;
use std::fmt;

fn control_err() -> ! {
    panic!("fail to run");
}
struct FileContent {
    content: Vec<u8>,
}
impl fmt::Display for FileContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for (index, element) in self.content.iter().enumerate() {
            result.push_str(format!("{:x} ", element).as_str());
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
        println!("\nread some array {:?}", x_vec);
    }
    {
        use std::io::prelude::*;
        use std::fs::File;
        let mut fs_create = File::create("hello.txt").expect("fail to crate");
        let mut insert_content: Vec<u8> = Vec::new();
        for _ in 0..10 {
            let number = rand::thread_rng().gen_range(0, 255);
            insert_content.push(number);
        }
        fs_create.write(&insert_content).expect("fail to write");
        let mut fs = match File::open("hello.txt") {
            Ok(file) => file,
            Err(_) => control_err(),
        };
        let mut content: Vec<u8> = Vec::new();
        match fs.read_to_end(&mut content) {
            Ok(_) => {
                println!("read complete");
            }
            Err(_) => {
                control_err();
            }
        }
        let i_have = FileContent { content: content };
        println!("here is the file content\n{}", i_have);
    }
    {
        use std::io::prelude::*;
        use std::fs::File;
        use std::io::{BufReader, BufWriter};
        {
            let fs = File::create("hello_buf.txt").unwrap();
            let mut writer = BufWriter::new(fs);
            match writer.write_fmt(format_args!("hello")) {
                Ok(_) => {
                    println!("read complete");
                }
                Err(_) => {
                    control_err();
                }
            }
        }
        {
            let fs = File::open("hello_buf.txt").unwrap();
            let mut reader = BufReader::new(fs);
            let mut content = String::new();
            match reader.read_to_string(&mut content) {
                Ok(_) => {
                    println!("read complete");
                }
                Err(_) => control_err(),
            }
            println!("{}", content);
        }
    }
}
