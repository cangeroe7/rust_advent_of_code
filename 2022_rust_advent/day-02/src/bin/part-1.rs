use day_02::process_part1;
use std::fs;    

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("\nPart 1: {}", process_part1(&input));
}