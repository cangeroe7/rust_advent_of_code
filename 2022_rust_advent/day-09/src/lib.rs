use std::collections::HashSet;

fn moves(head_xy: &mut (i32, i32), tail_xy: &mut (i32, i32), direction: &str, steps: i32) -> HashSet<(i32, i32)> {
    let mut locations: HashSet<(i32, i32)> = HashSet::new();
    match direction {
        "U" => {
            for _ in 0..steps {
                head_xy.1 += 1;
                if head_xy.1 - tail_xy.1 == 2 {
                    tail_xy.1 += 1;
                    tail_xy.0 = head_xy.0;
                    locations.insert(tail_xy.clone());
                }
            }
        },
        "D" => {
            for _ in 0..steps {
                head_xy.1 -= 1;
                if head_xy.1 - tail_xy.1 == -2 {
                    tail_xy.1 -= 1;
                    tail_xy.0 = head_xy.0;
                    locations.insert(tail_xy.clone());
                }
            }
        },
        "R" => {
            for _ in 0..steps {
                head_xy.0 += 1;
                if head_xy.0 - tail_xy.0 == 2 {
                    tail_xy.0 += 1;
                    tail_xy.1 = head_xy.1;
                    locations.insert(tail_xy.clone());
                }
            }
        },
        "L" => {
            for _ in 0..steps {
                head_xy.0 -= 1;
                if head_xy.0 - tail_xy.0 == -2 {
                    tail_xy.0 -= 1;
                    tail_xy.1 = head_xy.1;
                    locations.insert(tail_xy.clone());
                }
            }
        },
        &_ => todo!(),
    }
    locations
}



pub fn process_part1(input: &str) -> String {

    let mut head_xy: (i32, i32) = (0, 0);
    let mut tail_xy: (i32, i32) = (0, 0);
    let mut locations: HashSet<(i32, i32)> = HashSet::new();
    locations.insert((0, 0));
    let orders: Vec<(&str, i32)> = input
        .lines()
        .map(|line| {
            let (order, steps) = line.split_once(" ").unwrap();
            (order, steps.parse::<i32>().unwrap())
            })
        .collect();
    
    orders
        .iter()
        .for_each(|order| {
            locations.extend(moves(&mut head_xy, &mut tail_xy, order.0, order.1))
        });

    locations.len().to_string()
}



















// pub fn process_part2(input: &str) -> String {
//     "works".to_string()
// }