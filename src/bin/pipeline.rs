// pipeline.rs

use std::{fs::File, io::Read};

fn read_file(filename: &str) -> String {
    let mut data = String::new();
    {
        let mut file = File::open(filename).unwrap();
        file.read_to_string(&mut data).unwrap();
    }
    data
}

fn main() {
    let filename = "pride-and-prejudice.txt";
    let data = read_file(&filename);

    println!("ok");
}

// end
