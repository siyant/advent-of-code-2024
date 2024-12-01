use std::collections::HashMap;

advent_of_code::solution!(1);

fn read_left_right_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut left_entries: Vec<u32> = Vec::new();
    let mut right_entries: Vec<u32> = Vec::new();
    for line in lines {
        let line_split: Vec<&str> = line.split_whitespace().collect();
        let left = line_split[0].parse::<u32>().unwrap();
        let right = line_split[1].parse::<u32>().unwrap();
        left_entries.push(left);
        right_entries.push(right);
    }
    (left_entries, right_entries)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_list, mut right_list) = read_left_right_lists(input);
    left_list.sort();
    right_list.sort();

    let mut sum: u32 = 0;
    for i in 0..left_list.len() {
        let difference = left_list[i].abs_diff(right_list[i]);
        sum += difference;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_list, right_list) = read_left_right_lists(input);
    let mut right_count = HashMap::new();
    for entry in right_list {
        if let Some(count) = right_count.get_mut(&entry) {
            *count += 1;
        } else {
            right_count.insert(entry, 1 as u32);
        }
    }
    let mut similarity: u32 = 0;
    for entry in left_list {
        if let Some(count) = right_count.get_mut(&entry) {
            similarity += entry * *count;
        }
    }
    Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
