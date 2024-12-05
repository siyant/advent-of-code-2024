use std::collections::HashMap;

advent_of_code::solution!(5);

fn read_input(input: &str) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let split_index = input.lines().position(|line| line.is_empty()).unwrap();
    let rules: Vec<Vec<u32>> = input
        .lines()
        .take(split_index)
        .map(|line| line.split("|").map(|x| x.parse::<u32>().unwrap()).collect())
        .collect();
    let updates: Vec<Vec<u32>> = input
        .lines()
        .skip(split_index + 1)
        .map(|line| line.split(",").map(|x| x.parse::<u32>().unwrap()).collect())
        .collect();
    (rules, updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = read_input(input);

    let sum = updates
        .iter()
        .filter(|update| is_in_right_order(update, &rules))
        .map(|update| update.get((update.len() - 1) / 2).unwrap())
        .sum();

    Some(sum)
}

fn is_in_right_order(update: &Vec<u32>, rules: &Vec<Vec<u32>>) -> bool {
    let mut update_map = HashMap::new();
    for (index, value) in update.iter().enumerate() {
        update_map.insert(value, index);
    }
    for rule in rules {
        let left = rule[0];
        let right = rule[1];
        if let Some(left_index) = update_map.get(&left) {
            if let Some(right_index) = update_map.get(&right) {
                if left_index > right_index {
                    return false;
                }
            }
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = read_input(input);

    let sum = updates
        .iter()
        .filter(|update| !is_in_right_order(update, &rules))
        .map(|update| get_correct_ordering_for_update(update, &rules))
        .map(|corrected_update| {
            corrected_update
                .get((corrected_update.len() - 1) / 2)
                .unwrap()
                .clone()
        })
        .sum();

    Some(sum)
}

fn get_correct_ordering_for_update(update: &Vec<u32>, rules: &Vec<Vec<u32>>) -> Vec<u32> {
    let rules_for_update: Vec<Vec<u32>> = rules
        .iter()
        .filter(|rule| update.contains(&rule[0]) && update.contains(&rule[1]))
        .cloned()
        .collect();

    let mut bigger_count = HashMap::new();
    for rule in rules_for_update {
        bigger_count
            .entry(rule[1])
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut update_clone = update.clone();
    update_clone.sort_by(|a, b| {
        bigger_count
            .get(a)
            .unwrap_or(&0)
            .cmp(bigger_count.get(b).unwrap_or(&0))
    });
    update_clone
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
