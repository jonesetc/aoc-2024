use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");
const DIRECTION_OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

struct Map {
    cells: Vec<Vec<u8>>,
    size: (usize, usize),
}

impl Map {
    fn new(cells: Vec<Vec<u8>>) -> Self {
        let size = (cells.len(), cells[0].len());

        Self { cells, size }
    }
}

fn process(input: &str) -> Map {
    Map::new(
        input
            .trim()
            .split("\n")
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap_or(10) as u8)
                    .collect()
            })
            .collect(),
    )
}

fn list_endings(map: &Map, starting_point: (i32, i32), expected_value: u8) -> Vec<(i32, i32)> {
    if starting_point.0 < 0
        || starting_point.0 >= map.size.0 as i32
        || starting_point.1 < 0
        || starting_point.1 >= map.size.1 as i32
    {
        return vec![];
    }

    let starting_value = map.cells[starting_point.0 as usize][starting_point.1 as usize];

    if starting_value != expected_value {
        return vec![];
    } else if starting_value == 9 {
        return vec![starting_point];
    }

    DIRECTION_OFFSETS
        .iter()
        .flat_map(|offset| {
            list_endings(
                map,
                (starting_point.0 + offset.0, starting_point.1 + offset.1),
                expected_value + 1,
            )
        })
        .collect()
}

fn part1(input: &Map) -> impl ToString {
    input
        .cells
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, _)| {
                    list_endings(input, (i as i32, j as i32), 0)
                        .iter()
                        .collect::<HashSet<&(i32, i32)>>()
                        .len()
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn part2(input: &Map) -> impl ToString {
    input
        .cells
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, _)| list_endings(input, (i as i32, j as i32), 0).len())
                .sum::<usize>()
        })
        .sum::<usize>()
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
                    89010123\n\
                    78121874\n\
                    87430965\n\
                    96549874\n\
                    45678903\n\
                    32019012\n\
                    01329801\n\
                    10456732\n\
                "
            ))
            .to_string(),
            "36"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&process(
                "
                    89010123\n\
                    78121874\n\
                    87430965\n\
                    96549874\n\
                    45678903\n\
                    32019012\n\
                    01329801\n\
                    10456732\n\
                "
            ))
            .to_string(),
            "81"
        );
    }
}
