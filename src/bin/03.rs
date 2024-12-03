use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut result: u32 = 0;
    for capture in re.captures_iter(input) {
        let num1 = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let num2 = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
        result += num1 * num2;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut is_enabled = true;
    let mut result: u32 = 0;

    for capture in re.captures_iter(input) {
        let capture_str = capture.get(0).unwrap().as_str();
        match &capture_str[..3] {
            "mul" => {
                if is_enabled {
                    let num1 = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let num2 = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
                    result += num1 * num2;
                }
            }
            "do(" => {
                is_enabled = true;
            }
            "don" => {
                is_enabled = false;
            }
            _ => {}
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
