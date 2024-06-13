// drain.rs

use std::fs;

fn main() {
    let s = fs::read_to_string("./stop_words.txt").expect("this should be ok");
    let stop_words = s.split(",").collect::<Vec<&str>>();
    println!("{:?}", stop_words);
    println!("ok");
}

// end
