advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let input_vec: Vec<_> = input
        .lines()
        .map(|line| line.split("").filter(|x| *x != "").collect::<Vec<&str>>())
        .collect();
    let num_lines = input_vec.len();
    let num_per_line = input_vec[0].len();
    // println!("{:#?}", input_vec);

    let mut count: u32 = 0;
    for i in 0..num_lines {
        for j in 0..num_per_line {
            if input_vec[i][j] == "X" {
                count += count_xmas_from_x(&input_vec, i, j, num_lines, num_per_line);
            }
        }
    }
    Some(count)
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
const LETTERS: [&str; 3] = ["M", "A", "S"];

fn count_xmas_from_x(
    input_vec: &Vec<Vec<&str>>,
    i: usize,
    j: usize,
    num_lines: usize,
    num_per_line: usize,
) -> u32 {
    let mut count: u32 = 0;
    for (di, dj) in DIRECTIONS {
        if (i as i32) + 3 * di < 0
            || (i as i32) + 3 * di >= num_lines as i32
            || (j as i32) + 3 * dj < 0
            || (j as i32) + 3 * dj >= num_per_line as i32
        {
            // skip this direction as it will be out of range for the input_vec
            continue;
        }
        for l in 0..3 {
            let check_i = (i as i32) + (l + 1) * di;
            let check_j = (j as i32) + (l + 1) * dj;
            if input_vec[check_i as usize][check_j as usize] != LETTERS[l as usize] {
                break;
            }
            if l == 2 {
                count += 1;
            }
        }
    }
    count
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_vec: Vec<_> = input
        .lines()
        .map(|line| line.split("").filter(|x| *x != "").collect::<Vec<&str>>())
        .collect();
    let num_lines = input_vec.len();
    let num_per_line = input_vec[0].len();
    // println!("{:#?}", input_vec);

    let mut count: u32 = 0;
    for i in 1..(num_lines - 1) {
        for j in 1..(num_per_line - 1) {
            if input_vec[i][j] == "A" {
                count += count_xmas_from_a(&input_vec, i, j);
            }
        }
    }
    Some(count)
}

fn count_xmas_from_a(input_vec: &Vec<Vec<&str>>, i: usize, j: usize) -> u32 {
    let tl = input_vec[i - 1][j - 1];
    let tr = input_vec[i - 1][j + 1];
    let bl = input_vec[i + 1][j - 1];
    let br = input_vec[i + 1][j + 1];

    if (tl == "S" && br == "M" || tl == "M" && br == "S")
        && (tr == "S" && bl == "M" || tr == "M" && bl == "S")
    {
        return 1;
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
