use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx+1))
        .collect::<HashMap<char, usize>>();

    let result = input
        .lines()
        .map(|line| {
            let sack_length = line.len() / 2;
            let compartment_a = &line[0..sack_length];
            let compartment_b = &line[sack_length.. (sack_length * 2)];

            let common_char = compartment_a
                .chars()
                .find(|c| compartment_b.contains(*c))
                .unwrap();
            letter_scores.get(&common_char).unwrap()
        })
        .sum::<usize>();

    result.to_string()
}   

pub fn process_part2(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx+1))
        .collect::<HashMap<char, usize>>();

    let result = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|set| {
            let common_char = set[0]
            .chars()
            .find(|c| set[1].contains(*c) && set[2].contains(*c))
            .unwrap();
        letter_scores.get(&common_char).unwrap() 
        })
        .sum::<usize>();
    result.to_string()
}

