use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Box,
    LeftBox,
    RightBox,
    Empty,
    Wall,
    Robot,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Map {
    cells: Vec<Vec<Cell>>,
}

impl Map {
    fn doubled(&self) -> Self {
        let cells = self
            .cells
            .iter()
            .map(|row| {
                row.iter()
                    .flat_map(|cell| match cell {
                        Cell::Box => [Cell::LeftBox, Cell::RightBox],
                        Cell::Empty => [Cell::Empty, Cell::Empty],
                        Cell::Wall => [Cell::Wall, Cell::Wall],
                        Cell::Robot => [Cell::Robot, Cell::Empty],
                        Cell::LeftBox => unreachable!(),
                        Cell::RightBox => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        Self { cells }
    }

    fn get_cell(&self, coord: (usize, usize)) -> Cell {
        self.cells[coord.0][coord.1]
    }

    fn set_cell(&mut self, coord: (usize, usize), cell: Cell) {
        self.cells[coord.0][coord.1] = cell;
    }

    fn find_cells(&self, expected: Cell) -> Vec<(usize, usize)> {
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(
                        |(j, &cell)| {
                            if cell == expected {
                                Some((i, j))
                            } else {
                                None
                            }
                        },
                    )
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn find_robot_cell(&self) -> (usize, usize) {
        self.find_cells(Cell::Robot)[0]
    }

    fn get_shifts(
        &self,
        coord: (usize, usize),
        direction: Direction,
    ) -> Option<Vec<[(usize, usize); 2]>> {
        match self.get_cell(coord) {
            Cell::Box | Cell::Robot => Some(match direction {
                Direction::Up => vec![[coord, (coord.0 - 1, coord.1)]],
                Direction::Down => vec![[coord, (coord.0 + 1, coord.1)]],
                Direction::Left => vec![[coord, (coord.0, coord.1 - 1)]],
                Direction::Right => vec![[coord, (coord.0, coord.1 + 1)]],
            }),
            Cell::LeftBox => Some(match direction {
                Direction::Up => vec![
                    [coord, (coord.0 - 1, coord.1)],
                    [(coord.0, coord.1 + 1), (coord.0 - 1, coord.1 + 1)],
                ],
                Direction::Down => vec![
                    [coord, (coord.0 + 1, coord.1)],
                    [(coord.0, coord.1 + 1), (coord.0 + 1, coord.1 + 1)],
                ],
                Direction::Left => vec![[coord, (coord.0, coord.1 - 1)]],
                Direction::Right => vec![[coord, (coord.0, coord.1 + 1)]],
            }),
            Cell::RightBox => Some(match direction {
                Direction::Up => vec![
                    [coord, (coord.0 - 1, coord.1)],
                    [(coord.0, coord.1 - 1), (coord.0 - 1, coord.1 - 1)],
                ],
                Direction::Down => vec![
                    [coord, (coord.0 + 1, coord.1)],
                    [(coord.0, coord.1 - 1), (coord.0 + 1, coord.1 - 1)],
                ],
                Direction::Left => vec![[coord, (coord.0, coord.1 - 1)]],
                Direction::Right => vec![[coord, (coord.0, coord.1 + 1)]],
            }),
            Cell::Empty => Some(vec![]),
            Cell::Wall => None,
        }
        .and_then(|coord_pairs| {
            coord_pairs
                .iter()
                .map(|coord_pair| (coord_pair, self.get_shifts(coord_pair[1], direction)))
                .try_fold(
                    vec![],
                    |mut acc, (&coord_pair, maybe_to_move)| match maybe_to_move {
                        Some(mut to_move) => {
                            to_move.push(coord_pair);
                            acc.extend(to_move);

                            Some(acc)
                        }
                        _ => None,
                    },
                )
        })
    }
}

fn process(input: &str) -> (Map, Vec<Direction>) {
    let (cells_raw, directions_raw) = input.trim().split_once("\n\n").unwrap();

    let cells = cells_raw
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    'O' => Cell::Box,
                    '#' => Cell::Wall,
                    '@' => Cell::Robot,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let directions = directions_raw
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unreachable!(),
        })
        .collect();

    (Map { cells }, directions)
}

fn run_robot(mut map: Map, directions: &[Direction]) -> Map {
    directions.iter().for_each(|&direction| {
        let robot = map.find_robot_cell();
        if let Some(to_move) = map.get_shifts(robot, direction) {
            let mut seen = HashSet::new();
            to_move.into_iter().for_each(|coord_pair| {
                if !seen.insert(coord_pair) {
                    return;
                }

                map.set_cell(coord_pair[1], map.get_cell(coord_pair[0]));
                map.set_cell(coord_pair[0], Cell::Empty);
            });
        }
    });

    map
}

fn part1(input: &(Map, Vec<Direction>)) -> impl ToString {
    run_robot(input.0.clone(), &input.1)
        .find_cells(Cell::Box)
        .iter()
        .map(|(i, j)| 100 * i + j)
        .sum::<usize>()
}

fn part2(input: &(Map, Vec<Direction>)) -> impl ToString {
    run_robot(input.0.doubled(), &input.1)
        .find_cells(Cell::LeftBox)
        .iter()
        .map(|(i, j)| 100 * i + j)
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
                    ########\n\
                    #..O.O.#\n\
                    ##@.O..#\n\
                    #...O..#\n\
                    #.#.O..#\n\
                    #...O..#\n\
                    #......#\n\
                    ########\n\
                    \n\
                    <^^>>>vv<v>>v<<\n\
                "
            ))
            .to_string(),
            "2028"
        );

        assert_eq!(
            part1(&process(
                "
                    ##########\n\
                    #..O..O.O#\n\
                    #......O.#\n\
                    #.OO..O.O#\n\
                    #..O@..O.#\n\
                    #O#..O...#\n\
                    #O..O..O.#\n\
                    #.OO.O.OO#\n\
                    #....O...#\n\
                    ##########\n\
                    \n\
                    <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
                    vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
                    ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
                    <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
                    ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
                    ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
                    >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
                    <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
                    ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
                    v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\n\
                "
            ))
            .to_string(),
            "10092"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&process(
                "
                    ##########\n\
                    #..O..O.O#\n\
                    #......O.#\n\
                    #.OO..O.O#\n\
                    #..O@..O.#\n\
                    #O#..O...#\n\
                    #O..O..O.#\n\
                    #.OO.O.OO#\n\
                    #....O...#\n\
                    ##########\n\
                    \n\
                    <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
                    vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
                    ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
                    <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
                    ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
                    ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
                    >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
                    <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
                    ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
                    v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\n\
                "
            ))
            .to_string(),
            "9021"
        );
    }
}
