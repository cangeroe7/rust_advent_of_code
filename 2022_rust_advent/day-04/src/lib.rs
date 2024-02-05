struct Pair {
    left1: u32,
    right1: u32,
    left2: u32,
    right2: u32,
}

impl Pair {
    fn fits_whole(&self) -> u32 {
        ((self.left1 >= self.left2 && self.right1 <= self.right2) || 
        (self.left2 >= self.left1 && self.right2 <= self.right1)) as u32
    }

    fn overlaps(&self) -> u32 {
        ((self.left1 >= self.left2 && self.left1 <= self.right2) ||
        (self.right1 >= self.left2 && self.right1 <= self.right2) ||
        (self.left1 <= self.left2 && self.right1 >= self.right2)) as u32
    }
}

pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let split: Vec<u32> = line.split("-").map(|kar| kar.parse::<u32>().unwrap()).collect();
            let check = Pair {
                left1: split[0] as u32,
                right1: split[1] as u32,
                left2: split[2] as u32,
                right2: split[3] as u32,
            };
            check.fits_whole()
        }).sum::<u32>().to_string()
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let split: Vec<u32> = line.split("-").map(|kar| kar.parse::<u32>().unwrap()).collect();
            let check = Pair {
                left1: split[0] as u32,
                right1: split[1] as u32,
                left2: split[2] as u32,
                right2: split[3] as u32,
            };
            check.overlaps()
        }).sum::<u32>().to_string()
}