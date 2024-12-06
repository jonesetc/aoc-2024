use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

const DIRECTION_OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[allow(clippy::type_complexity)]
fn process(input: &str) -> ((usize, usize), (usize, usize), HashSet<(usize, usize)>) {
    let mut grid_size: (usize, usize) = (0, 0);
    let mut guard: Option<(usize, usize)> = None;
    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();

    input.trim().split("\n").enumerate().for_each(|(i, line)| {
        grid_size.0 += 1;
        line.char_indices().for_each(|(j, symbol)| {
            grid_size.1 += 1;
            if symbol == '^' {
                guard = Some((i, j));
            } else if symbol == '#' {
                obstacles.insert((i, j));
            };
        });
    });

    (grid_size, guard.unwrap(), obstacles)
}

fn walk(
    grid_size: &(usize, usize),
    starting_pos: &(usize, usize),
    obstacles: &HashSet<(usize, usize)>,
) -> (bool, HashSet<(usize, usize)>) {
    let mut guard_dir = 0usize;
    let mut guard_pos = *starting_pos;
    let mut seen: HashSet<(usize, (usize, usize))> = HashSet::new();

    let looped = loop {
        if !seen.insert((guard_dir, guard_pos)) {
            break true;
        }

        let next_pos = (
            guard_pos.0 as i32 + DIRECTION_OFFSETS[guard_dir].0,
            guard_pos.1 as i32 + DIRECTION_OFFSETS[guard_dir].1,
        );

        if next_pos.0 < 0
            || next_pos.0 >= grid_size.0 as i32
            || next_pos.1 < 0
            || next_pos.1 >= grid_size.1 as i32
        {
            break false;
        }

        if obstacles.contains(&(next_pos.0 as usize, next_pos.1 as usize)) {
            guard_dir = (guard_dir + 1) % 4;
        } else {
            guard_pos = (next_pos.0 as usize, next_pos.1 as usize);
        }
    };

    (looped, seen.iter().map(|&(_, pos)| pos).collect())
}

#[allow(clippy::type_complexity)]
fn part1(input: &((usize, usize), (usize, usize), HashSet<(usize, usize)>)) -> impl ToString {
    walk(&input.0, &input.1, &input.2).1.len()
}

#[allow(clippy::type_complexity)]
fn part2(input: &((usize, usize), (usize, usize), HashSet<(usize, usize)>)) -> impl ToString {
    let seen = walk(&input.0, &input.1, &input.2).1;

    seen.iter()
        .filter(|&&pos| {
            if pos == input.1 {
                return false;
            }

            let mut obstacles = input.2.clone();
            obstacles.insert(pos);

            walk(&input.0, &input.1, &obstacles).0
        })
        .count()
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
                    ....#.....\n\
                    .........#\n\
                    ..........\n\
                    ..#.......\n\
                    .......#..\n\
                    ..........\n\
                    .#..^.....\n\
                    ........#.\n\
                    #.........\n\
                    ......#...\n\
                "
            ))
            .to_string(),
            "41",
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&process(
                "
                    ....#.....\n\
                    .........#\n\
                    ..........\n\
                    ..#.......\n\
                    .......#..\n\
                    ..........\n\
                    .#..^.....\n\
                    ........#.\n\
                    #.........\n\
                    ......#...\n\
                "
            ))
            .to_string(),
            "6",
        );
    }
}
