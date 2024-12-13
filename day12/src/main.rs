use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");
const CORNER_PAIRS: [[usize; 2]; 4] = [[0, 2], [0, 3], [1, 2], [1, 3]];

struct Map {
    cells: Vec<Vec<char>>,
    size: (usize, usize),
}

impl Map {
    fn new(cells: Vec<Vec<char>>) -> Self {
        let size = (cells.len(), cells[0].len());

        Self { cells, size }
    }

    fn get(&self, coord: &(usize, usize)) -> char {
        self.cells[coord.0][coord.1]
    }

    fn get_neighbors(&self, coord: &(usize, usize)) -> [Option<(usize, usize)>; 4] {
        [
            if coord.0 > 0 {
                Some((coord.0 - 1, coord.1))
            } else {
                None
            },
            if coord.0 + 1 < self.size.0 {
                Some((coord.0 + 1, coord.1))
            } else {
                None
            },
            if coord.1 > 0 {
                Some((coord.0, coord.1 - 1))
            } else {
                None
            },
            if coord.1 + 1 < self.size.1 {
                Some((coord.0, coord.1 + 1))
            } else {
                None
            },
        ]
    }

    fn discover_plot(
        &self,
        coord: &(usize, usize),
        seen: &mut HashSet<(usize, usize)>,
        expected: char,
    ) -> (usize, usize, usize) {
        let current = self.get(coord);
        if expected != current {
            return (0, 1, 0);
        }

        seen.insert(*coord);

        let neighbors = self.get_neighbors(coord);

        let corners = CORNER_PAIRS
            .iter()
            .filter(
                |indexes| match (neighbors[indexes[0]], neighbors[indexes[1]]) {
                    (None, None) => true,
                    (None, Some(other)) => self.get(&other) != current,
                    (Some(other), None) => self.get(&other) != current,
                    (Some(first_other), Some(second_other)) => {
                        let others = (self.get(&first_other), self.get(&second_other));

                        (others.0 != current && others.1 != current)
                            || (others.0 == current
                                && others.1 == current
                                && self.get(&self.get_neighbors(&first_other)[indexes[1]].unwrap())
                                    != current)
                    }
                },
            )
            .count();

        neighbors
            .into_iter()
            .map(|maybe_neighbor| {
                if let Some(neighbor) = maybe_neighbor {
                    if seen.contains(&neighbor) {
                        (0, (self.get(&neighbor) != expected) as usize, 0)
                    } else {
                        self.discover_plot(&neighbor, seen, current)
                    }
                } else {
                    (0, 1, 0)
                }
            })
            .fold((1, 0, corners), |acc, curr| {
                (acc.0 + curr.0, acc.1 + curr.1, acc.2 + curr.2)
            })
    }

    fn discover_all_plots(&self) -> Vec<(usize, usize, usize)> {
        let mut seen = HashSet::new();
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, &plant)| {
                        if seen.contains(&(i, j)) {
                            None
                        } else {
                            Some(self.discover_plot(&(i, j), &mut seen, plant))
                        }
                    })
                    .collect::<Vec<(usize, usize, usize)>>()
            })
            .collect()
    }
}

fn process(input: &str) -> Map {
    Map::new(
        input
            .trim()
            .split("\n")
            .map(|line| line.chars().collect())
            .collect(),
    )
}

fn part1(input: &Map) -> impl ToString {
    input
        .discover_all_plots()
        .iter()
        .fold(0, |acc, curr| acc + curr.0 * curr.1)
}

fn part2(input: &Map) -> impl ToString {
    input
        .discover_all_plots()
        .iter()
        .fold(0, |acc, curr| acc + curr.0 * curr.2)
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
                    AAAA\n\
                    BBCD\n\
                    BBCC\n\
                    EEEC\n\
                "
            ))
            .to_string(),
            "140"
        );

        assert_eq!(
            part1(&process(
                "
                    OOOOO\n\
                    OXOXO\n\
                    OOOOO\n\
                    OXOXO\n\
                    OOOOO\n\
                "
            ))
            .to_string(),
            "772"
        );

        assert_eq!(
            part1(&process(
                "
                    RRRRIICCFF\n\
                    RRRRIICCCF\n\
                    VVRRRCCFFF\n\
                    VVRCCCJFFF\n\
                    VVVVCJJCFE\n\
                    VVIVCCJJEE\n\
                    VVIIICJJEE\n\
                    MIIIIIJJEE\n\
                    MIIISIJEEE\n\
                    MMMISSJEEE\n\
                "
            ))
            .to_string(),
            "1930"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&process(
                "
                    AAAA\n\
                    BBCD\n\
                    BBCC\n\
                    EEEC\n\
                "
            ))
            .to_string(),
            "80"
        );

        assert_eq!(
            part2(&process(
                "
                    OOOOO\n\
                    OXOXO\n\
                    OOOOO\n\
                    OXOXO\n\
                    OOOOO\n\
                "
            ))
            .to_string(),
            "436"
        );

        assert_eq!(
            part2(&process(
                "
                    EEEEE\n\
                    EXXXX\n\
                    EEEEE\n\
                    EXXXX\n\
                    EEEEE\n\
                "
            ))
            .to_string(),
            "236"
        );

        assert_eq!(
            part2(&process(
                "
                    AAAAAA\n\
                    AAABBA\n\
                    AAABBA\n\
                    ABBAAA\n\
                    ABBAAA\n\
                    AAAAAA\n\
                "
            ))
            .to_string(),
            "368"
        );

        assert_eq!(
            part2(&process(
                "
                    RRRRIICCFF\n\
                    RRRRIICCCF\n\
                    VVRRRCCFFF\n\
                    VVRCCCJFFF\n\
                    VVVVCJJCFE\n\
                    VVIVCCJJEE\n\
                    VVIIICJJEE\n\
                    MIIIIIJJEE\n\
                    MIIISIJEEE\n\
                    MMMISSJEEE\n\
                "
            ))
            .to_string(),
            "1206"
        );
    }
}
