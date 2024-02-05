use std::collections::VecDeque;

pub fn process_part(input: &str, kars: usize) -> String {
    let mut check: VecDeque<char> = VecDeque::new();
    let enum_chars = input.chars().enumerate();
    for (idx, kar) in enum_chars {
        while check.contains(&kar) {
            check.pop_back();
        }
        check.push_front(kar);
        if check.len() == kars {
            return (idx+1).to_string();
        }
    }
    unreachable!()
}