const INPUT: &str = include_str!("../input.txt");
const DIRECTION_OFFSETS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

fn process(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect()
}

#[allow(clippy::ptr_arg)]
fn check_coordinate(grid: &Vec<Vec<char>>, i: i32, j: i32, expected: char) -> bool {
    if i < 0 || j < 0 {
        return false;
    }

    grid.get(i as usize)
        .and_then(|row| row.get(j as usize))
        .map_or(false, |&cell| cell == expected)
}

fn part1(input: &Vec<Vec<char>>) -> impl ToString {
    let mut count = 0;
    for i in 0..(input.len() as i32) {
        for j in 0..(input[0].len() as i32) {
            if check_coordinate(input, i, j, 'X') {
                'direction: for offset in DIRECTION_OFFSETS.iter() {
                    let mut coords = (i, j);
                    for c in "MAS".chars() {
                        coords = (coords.0 + offset.0, coords.1 + offset.1);
                        if !check_coordinate(input, coords.0, coords.1, c) {
                            continue 'direction;
                        }
                    }
                    count += 1
                }
            }
        }
    }

    count
}

fn part2(input: &Vec<Vec<char>>) -> impl ToString {
    let mut count = 0;
    for i in 0..(input.len() as i32) {
        for j in 0..(input[0].len() as i32) {
            if check_coordinate(input, i, j, 'A')
                && ((check_coordinate(input, i - 1, j + 1, 'M')
                    && check_coordinate(input, i + 1, j - 1, 'S'))
                    || (check_coordinate(input, i - 1, j + 1, 'S')
                        && check_coordinate(input, i + 1, j - 1, 'M')))
                && ((check_coordinate(input, i - 1, j - 1, 'M')
                    && check_coordinate(input, i + 1, j + 1, 'S'))
                    || (check_coordinate(input, i - 1, j - 1, 'S')
                        && check_coordinate(input, i + 1, j + 1, 'M')))
            {
                count += 1
            }
        }
    }

    count
}

fn main() {
    println!("Processing input");
    let input = process(INPUT);
    println!("------");

    println!("Running part 1");
    println!("Result: {}", part1(&input).to_string());
    println!("------");

    println!("Running part 2");
    println!("Result: {}", part2(&input).to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&process(
                "
                    MMMSXXMASM\n\
                    MSAMXMSMSA\n\
                    AMXSXMAAMM\n\
                    MSAMASMSMX\n\
                    XMASAMXAMM\n\
                    XXAMMXXAMA\n\
                    SMSMSASXSS\n\
                    SAXAMASAAA\n\
                    MAMMMXMMMM\n\
                    MXMXAXMASX\n\
                "
            ))
            .to_string(),
            "18",
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&process(
                "
                    MMMSXXMASM\n\
                    MSAMXMSMSA\n\
                    AMXSXMAAMM\n\
                    MSAMASMSMX\n\
                    XMASAMXAMM\n\
                    XXAMMXXAMA\n\
                    SMSMSASXSS\n\
                    SAXAMASAAA\n\
                    MAMMMXMMMM\n\
                    MXMXAXMASX\n\
                "
            ))
            .to_string(),
            "9",
        );
    }
}
