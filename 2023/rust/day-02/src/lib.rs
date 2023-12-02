use std::string::ParseError;
use std::str::FromStr;
use std::cmp;

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Cubes>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Cubes {
    r: u32,
    g: u32,
    b: u32,
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();

        let id: u32 = parts[0][5..].parse::<u32>().unwrap();
        
        let sets_str: Vec<&str> = parts[1].split(';').collect();

        let mut sets = Vec::new();

        for set in sets_str {
            sets.push(Cubes::from_str(set).unwrap());
        }

        Ok(Game { id, sets })
    }
}

impl FromStr for Cubes {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs: Vec<&str> = s.split(',').collect();

        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for pair in pairs {
            let mut iter = pair.split_whitespace();
            let value = iter.next().expect("Expected a value");
            let key = iter.next().expect("Expected a key");

            if key == "red" {
                if value.parse::<u32>().unwrap() > r {
                    r = value.parse::<u32>().unwrap();                
                }
            } else if key.eq("green") {
                if value.parse::<u32>().unwrap() > g {
                    g = value.parse::<u32>().unwrap();                
                }
            } else if key.eq("blue") {
                if value.parse::<u32>().unwrap() > b {
                    b = value.parse::<u32>().unwrap();                
                }
            }
        }
        Ok(Cubes { r, g, b })
    }
}

pub fn process_part1(input: &str) -> String {
    let max = Cubes { r: 12, g: 13, b: 14 };
    
    let games = input.lines()
        .map(|game| Game::from_str(game).unwrap());

    let mut total = 0;

    for game in games {
        let mut valid = true;
        for set in game.sets {
            if set.r > max.r || set.g > max.g || set.b > max.b {
                valid = false;
                break;
            }
        }
        if valid { 
           total += game.id;
        }
    }
    total.to_string()
}

pub fn process_part2(input: &str) -> String {

    let games = input.lines()
        .map(|game| Game::from_str(game).unwrap());

    let mut total = 0;

    for game in games {
        let mut min = Cubes { r: 0, g: 0, b: 0 };
        for set in game.sets {
            min.r = cmp::max(min.r, set.r);
            min.g = cmp::max(min.g, set.g);
            min.b = cmp::max(min.b, set.b);
        }
        total += min.r * min.g * min.b;
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {

        let result =process_part1(INPUT);
        assert_eq!(result, "8");
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "2286");
    }
}
