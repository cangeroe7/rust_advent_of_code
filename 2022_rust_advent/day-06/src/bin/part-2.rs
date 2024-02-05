use day_06::process_part;
use std::fs;    

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("\nPart 2: {}", process_part(&input, 14));
}