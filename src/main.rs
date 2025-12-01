use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");
    day_one();
}

fn day_one() {
    let source = fs::read_to_string("/home/gizu/work/advent-of-code/assets/dayone.txt")
        .expect("Should have been able to read contents of file.");
}
