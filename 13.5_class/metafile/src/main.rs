use std::io::{self, BufReader, BufWriter};
use std::io::prelude::*;
use std::fs::{self, OpenOptions};
use std::path::Path;
fn conerror() -> ! {
    panic!("fail to run");
}
fn file_breif(file_name: &Path) {
    let meta_file = fs::metadata(file_name).unwrap();
    let fs = match OpenOptions::new().read(true).open(file_name) {
        Ok(file) => {
            println!("read complete");
            file
        }
        Err(_) => conerror(),
    };
    let mut content = String::new();
    {
        BufReader::new(fs).read_to_string(&mut content).expect("fail to read");
    }
    println!("\nhere is the file_breif");
    println!("File Name: {:?}", file_name.file_name().unwrap());
    println!("File Type: {:?}", meta_file.file_type());
    println!("File Size: {}", meta_file.len());
    println!("File Content:\n{}", content);
    println!("Recent modified: {:?}", meta_file.modified().unwrap());

}
fn main() {
    let file_name = Path::new("metafile_test");
    let fs = match OpenOptions::new().write(true).create(true).open(file_name) {
        Ok(file) => {
            println!("read complete");
            file
        }
        Err(_) => conerror(),
    };
    {
        let mut content = String::new();
        io::stdin().read_line(&mut content).expect("fail to read");
        match BufWriter::new(fs).write_fmt(format_args!("i wrote {}", content.trim())) {
            Ok(_) => {
                println!("write complete");
            }
            Err(_) => {
                conerror();
            }
        };
    }
    file_breif(&file_name);
}
