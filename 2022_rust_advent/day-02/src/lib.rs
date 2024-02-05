use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
    let combo_score = HashMap::from([
        ("AX", 4),
        ("BX", 1),
        ("CX", 7),
        ("AY", 8),
        ("BY", 5),
        ("CY", 2),
        ("AZ", 3),
        ("BZ", 9),
        ("CZ", 6),
    ]);

    let results = input
        .split("\r\n")
        .map(|set| combo_score[set])
        .sum::<i32>();
    results.to_string() 
}

pub fn process_part2(input: &str) -> String{
    let combo_score = HashMap::from([
        ("AX", 3),
        ("BX", 1),
        ("CX", 2),
        ("AY", 4),
        ("BY", 5),
        ("CY", 6),
        ("AZ", 8),
        ("BZ", 9),
        ("CZ", 7),
    ]);

    let results = input
        .split("\r\n")
        .map(|set| combo_score[set])
        .sum::<i32>();
    results.to_string() 
}