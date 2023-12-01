pub fn process_part1(input: &str) -> String {
    let mut result = Vec::new();

    for line in input.lines() {
        let numeric_chars: String = line.chars().filter(|c| c.is_numeric()).collect();
        result.push(numeric_chars)
    }

    for line in &mut result {
        let n1 = line.chars().next().unwrap();
        let n2 = line.chars().last().unwrap();
        *line = format!("{}{}", n1, n2);
    }
    
    let sum: u32 = result.iter()
        .map(|item| item.parse::<u32>().unwrap())
        .sum();
    sum.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result = input.lines().map(|line| {
        if line.len() == 0 {
            return 0;
        }
        let mut first_idx = line.len();
        let mut last_idx = 0;
        let mut first_num = 0;
        let mut last_num = 0;
        for num in 1..=9 {
            let word = match num {
                1 => "one",
                2 => "two",
                3 => "three",
                4 => "four",
                5 => "five",
                6 => "six",
                7 => "seven",
                8 => "eight",
                _ => "nine",
            };
            // Check if the first num exists as letters and is at an idx before current first num
            let mut word_idx = line.find(word);
            if word_idx.is_some() && word_idx.unwrap() <= first_idx {
                first_idx = word_idx.unwrap();
                first_num = num;
            }
            // Check if first number exists as a number and is at an idx before current first num
            let mut num_idx = line.find(&num.to_string());
            if num_idx.is_some() && num_idx.unwrap() <= first_idx {
                first_idx = num_idx.unwrap();
                first_num = num;
            }
            // Check if the last num exists as letters and is at an idx before current last num
            word_idx = line.rfind(word);
            if word_idx.is_some() && word_idx.unwrap() >= last_idx {
                last_idx = word_idx.unwrap();
                last_num = num;
            }
            // Check if last number exists as a number and is at an idx before current last num
            num_idx = line.rfind(&num.to_string());
            if num_idx.is_some() && num_idx.unwrap() >= last_idx {
                last_idx = num_idx.unwrap();
                last_num = num;
            }
        }
        first_num * 10 + last_num
        
    }).sum::<i32>();
    result.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn part1() {
        let input: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = process_part1(input);
        assert_eq!(result, "142");
    }

    #[test]
    fn part2() {
        let input: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = process_part2(input);
        assert_eq!(result, "281");
    }
}
