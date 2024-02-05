pub fn process_part1(input: &str) -> String{
    let result = input
            .split("\r\n\r\n")
            .map(|elf_load| {
                elf_load
                    .lines()
                    .map(|item| item.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .max()
            .unwrap();
    result.to_string()         
}

pub fn process_part2(input: &str) -> String {
    let mut result = input
            .split("\r\n\r\n")
            .map(|elf_load| {
                elf_load
                    .lines()
                    .map(|item| item.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .collect::<Vec<_>>();
    result.sort_by(|a, b| b.cmp(a));
    let top_total = result[0] + result[1] + result[2];
    top_total.to_string()
}

