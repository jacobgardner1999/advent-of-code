pub fn process_part1(input: &str) -> String {
    let data: Vec<(Vec<u32>, Vec<u32>)> = input
        .lines()
        .map(|line| {
            let halves: Vec<&str> = line.split('|').collect();
            let left_title: Vec<&str> = halves[0].split(':').collect();
            let left: Vec<u32> = left_title[1]
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let right: Vec<u32> = halves[1]
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            (left, right)
        }).collect();

    let mut total = 0;

    for game in data {
        let mut matches: i32 = -1;
        for num in &game.0 {
            if game.1.contains(&num) {
                matches += 1;
            }
        }
        if matches != -1 {
            let base: i32 = 2;
            total += base.pow(matches as u32);
        }
    }

    total.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut data: Vec<(u32, (Vec<u32>, Vec<u32>))> = input
        .lines()
        .map(|line| {
            let halves: Vec<&str> = line.split('|').collect();
            let left_all: Vec<&str> = halves[0].split(':').collect();
            let id: u32= left_all[0]
                .split_whitespace()
                .last().unwrap()
                .parse().unwrap();
            let left: Vec<u32> = left_all[1]
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let right: Vec<u32> = halves[1]
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            (id, (left, right))
        }).collect();

    let mut idx = 0;

    while idx < data.len() {
        let mut matches = 0;
        for num in &data[idx].1 .0 {
            if data[idx].1 .1.contains(&num) {
                matches += 1;
            }
        }

        for n in 1..=matches {
            let new_idx = (data[idx].0 + n - 1) as usize;
            data.push(data[new_idx].clone());
        }
        idx += 1;
    }
    data.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        let result =process_part1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "30");
    }
}

