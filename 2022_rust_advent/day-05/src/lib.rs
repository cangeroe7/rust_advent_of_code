const STACK: usize = 9;

pub fn process_part1(input: &str) -> String {
    let mut boxes_hold: [Vec<char>; STACK] = Default::default();
    let (boxes, orders_str) = input.split_once("\r\n\r\n").unwrap();

    boxes
        .lines()
        .rev()
        .skip(1)
        .for_each(|line| {
            line
                .chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, kar)| *kar != ' ')
                .for_each(|(idx, kar)| boxes_hold[idx].push(kar))
            
        });
    let orders: Vec<Vec<usize>> = orders_str
        .lines()
        .map(|line| {
            line
                .split(" ")
                .skip(1)
                .step_by(2)
                .map(|part| part.trim().parse::<usize>().unwrap())
                .collect()
        }).collect();
    
    orders
        .iter()
        .for_each(|order| {
            for _ in 0..order[0] {
                let tmp = boxes_hold[order[1] - 1].pop().unwrap();
                boxes_hold[order[2] - 1].push(tmp);
            }
        });
    let mut answer = String::new();
    boxes_hold
        .iter()
        .for_each(|list| if !list.is_empty() {answer.push(*list.last().unwrap())});
    answer
}

pub fn process_part2(input: &str) -> String {
    let mut boxes_hold: [Vec<char>; STACK] = Default::default();
    let (boxes, orders_str) = input.split_once("\r\n\r\n").unwrap();

    boxes
        .lines()
        .rev()
        .skip(1)
        .for_each(|line| {
            line
                .chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, kar)| *kar != ' ')
                .for_each(|(idx, kar)| boxes_hold[idx].push(kar))    
        });

    let orders: Vec<Vec<usize>> = orders_str
        .lines()
        .map(|line| {
            line
                .split(" ")
                .skip(1)
                .step_by(2)
                .map(|part| part.trim().parse::<usize>().unwrap())
                .collect()
        }).collect();
    
    orders
        .iter()
        .for_each(|order| {
            let mut hold = Vec::new();
            for _ in 0..order[0] {
                hold.push(boxes_hold[order[1] - 1].pop().unwrap());   
            }
            for _ in 0..order[0] {
                let tmp = hold.pop().unwrap();
                boxes_hold[order[2] - 1].push(tmp);
            }
        });

    let mut answer = String::new();
    boxes_hold
        .iter()
        .for_each(|list| if !list.is_empty() {answer.push(*list.last().unwrap())});
    answer
}
