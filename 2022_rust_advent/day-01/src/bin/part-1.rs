use day_01::process_part1;
use std::fs;


fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();  //.replace("\r", "");
    println!("Part 1: {}", process_part1(&input));
}