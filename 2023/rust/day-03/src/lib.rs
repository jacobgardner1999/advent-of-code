const SYMBOLS: [char; 11] = ['!','#','$','%','&','*','@','/','=','+','-'];

fn symbol_scan(data: &Vec<Vec<char>>, line: usize, start_idx: usize, end_idx: usize) -> bool {
    //check top left
    if line > 0 && start_idx > 0 {
        let element = data.get(line - 1).unwrap().get(start_idx - 1).unwrap();
        if SYMBOLS.contains(element) {
            return true
        }
    }
    //check left
    if start_idx > 0 {
        let element = data.get(line).unwrap().get(start_idx - 1).unwrap();
        if SYMBOLS.contains(element) {
            return true
        }
    }
    
    //check bottom left
    if line < data.len() - 1 && start_idx > 0 {
        let element = data.get(line + 1).unwrap().get(start_idx - 1).unwrap();
        if SYMBOLS.contains(element) {
            return true
        }
    }

    //check top
    if line > 0 {
        for n in start_idx..=end_idx {
            let element = data.get(line - 1).unwrap().get(n).unwrap();
            if SYMBOLS.contains(element) {
                return true
            }
        }
    }
    
    //check top right
    if line > 0 && end_idx < data.get(line).unwrap().len() - 1 {
        let element = data.get(line - 1).unwrap().get(end_idx + 1).unwrap();
        if SYMBOLS.contains(element) {
            return true
        }
    }
    
    //check right
    if end_idx < data.get(line).unwrap().len() - 1 {
        let element = data.get(line).unwrap().get(end_idx + 1).unwrap();
        if SYMBOLS.contains(element) {
            return true
        }
    }
    
    //check bottom right
    if line < data.len() - 1 && end_idx < data.get(line).unwrap().len() - 1 {
        let element = data.get(line + 1).unwrap().get(end_idx + 1).unwrap();
        if SYMBOLS.contains(element) {
            return true
        }
    }
    
    //check bottom
    if line < data.len() - 1{
        for n in start_idx..= end_idx {
            let element = data.get(line + 1).unwrap().get(n).unwrap();
            if SYMBOLS.contains(element) {
                return true
            }
        }
    }
    false
}
pub fn process_part1(input: &str) -> String {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut total = 0;
    let mut curr_line = 0;
    let mut prev = '.';
    let mut num_str: String = "".to_owned();
    let mut start_idx = 0;
    let mut end_idx = 0;
    let mut prev_idx = 0;

    for line in &data {
        
        let mut idx = 0;
        for char in line {
            if char.is_digit(10) {
                if !prev.is_digit(10) {
                    start_idx = idx;
                }
                num_str += char.to_string().as_str();
            } else if prev.is_digit(10) {
                end_idx = prev_idx;
                if idx == 0 {
                    if symbol_scan(&data, curr_line - 1, start_idx, end_idx) {
                        total += num_str.parse::<u32>().unwrap();
                        dbg!(&curr_line, &num_str);
                    }
                } else {
                    if symbol_scan(&data, curr_line, start_idx, end_idx) {
                        total += num_str.parse::<u32>().unwrap();
                        dbg!(&curr_line, &num_str);
                    }
                }
                num_str = "".to_owned();
            }
            prev_idx = idx;
            idx += 1;
            prev = *char;
        }
        curr_line += 1;
    }

    total.to_string()
}

pub fn process_part2(_input: &str) -> String {
    "Implement part 2".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_symbol_scan() {
        let data: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
        let result = symbol_scan(&data, 0, 0, 2); 
        assert_eq!(result, true);
    }

    #[test]
    fn test_top_left() {
        let input = "*...
.46.
....";
        let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let result = symbol_scan(&data, 1, 1, 2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_top() {
        let input = "..#.
.46.
....";
        let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let result = symbol_scan(&data, 1, 1, 2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_top_right() {
        let input = "...&
.46.
....";
        let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let result = symbol_scan(&data, 1, 1, 2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_right() {
        let input = "....
.46$
....";
        let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let result = symbol_scan(&data, 1, 1, 2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_bottom_right() {
        let input = "....
.46.
.../";
        let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let result = symbol_scan(&data, 1, 1, 2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_bottom() {
        let input = "....
.46.
../.";
        let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let result = symbol_scan(&data, 1, 1, 2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_bottom_left() {
        let input = "....
.46.
=...";
        let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let result = symbol_scan(&data, 1, 1, 2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_left() {
        let input = "....
+46.
....";
        let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let result = symbol_scan(&data, 1, 1, 2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_end_of_line() {
       let input = "....$
..235
.....";
        let result = process_part1(input);
        assert_eq!(result, "235");
    }
    
    #[test]
    fn test_start_of_line() {
       let input = ".....
235!.
.....";
        let result = process_part1(input);
        assert_eq!(result, "235");
    }
    #[test]
    fn test_part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "4361");
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "");
    }
}
