advent_of_code::solution!(2);

fn read_reports(input: &str) -> Vec<Vec<u32>> {
    let reports = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|level| level.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    // println!("{:?}", reports);
    reports
}

fn is_safe_report(report: &Vec<u32>) -> bool {
    let mut direction = "unknown";
    for level_idx in 1..report.len() {
        let prev = report[level_idx - 1];
        let current = report[level_idx];
        if current > prev {
            if direction == "decreasing" {
                return false;
            }
            direction = "increasing";
        } else if current < prev {
            if direction == "increasing" {
                return false;
            }
            direction = "decreasing";
        } else {
            return false;
        }
        if current.abs_diff(prev) > 3 {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let count = read_reports(input)
        .into_iter()
        .filter(|report| is_safe_report(report))
        .count() as u32;
    Some(count)
}

fn is_safe_report_fail_level(report: &Vec<u32>) -> (bool, usize) {
    let mut direction = "unknown";
    for level_idx in 1..report.len() {
        let prev = report[level_idx - 1];
        let current = report[level_idx];
        if current > prev {
            if direction == "decreasing" {
                return (false, level_idx);
            }
            direction = "increasing";
        } else if current < prev {
            if direction == "increasing" {
                return (false, level_idx);
            }
            direction = "decreasing";
        } else {
            return (false, level_idx);
        }
        if current.abs_diff(prev) > 3 {
            return (false, level_idx);
        }
    }
    (true, 0)
}

fn is_safe_report_with_dampener(report: &Vec<u32>) -> bool {
    let (is_original_safe, unsafe_level) = is_safe_report_fail_level(&report);
    if is_original_safe {
        return true;
    }
    for idx in [0, 1, unsafe_level] {
        let mut report_copy = report.clone();
        report_copy.remove(idx);
        let (is_removed_safe, _) = is_safe_report_fail_level(&report_copy);
        if is_removed_safe {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let count = read_reports(input)
        .into_iter()
        .filter(|report| is_safe_report_with_dampener(report))
        .count() as u32;
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
