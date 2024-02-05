use day_09::process_part2;
use std::fs;    

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("\nPart 2: {}", process_part2(&input));
}