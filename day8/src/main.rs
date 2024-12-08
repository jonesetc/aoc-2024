use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

#[allow(clippy::type_complexity)]
fn process(input: &str) -> ((i32, i32), HashMap<char, HashSet<(i32, i32)>>) {
    let mut grid_size: (i32, i32) = (0, 0);
    let mut antennae: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();

    input.trim().split("\n").enumerate().for_each(|(i, line)| {
        grid_size.0 += 1;
        line.char_indices().for_each(|(j, c)| {
            if grid_size.0 == 1 {
                grid_size.1 += 1;
            }

            if c.is_alphanumeric() {
                antennae.entry(c).or_default().insert((i as i32, j as i32));
            }
        });
    });

    (grid_size, antennae)
}

fn is_in_bounds(maxes: (i32, i32), coord: (i32, i32)) -> bool {
    coord.0 >= 0 && coord.0 < maxes.0 && coord.1 >= 0 && coord.1 < maxes.1
}

#[allow(clippy::type_complexity)]
fn part1(input: &((i32, i32), HashMap<char, HashSet<(i32, i32)>>)) -> impl ToString {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    input.1.values().for_each(|antennae| {
        antennae.iter().for_each(|antenna| {
            antennae.iter().for_each(|other| {
                if antenna == other {
                    return;
                }

                let (i_diff, i_cmp) = (antenna.0.abs_diff(other.0) as i32, antenna.0.cmp(&other.0));
                let (j_diff, j_cmp) = (antenna.1.abs_diff(other.1) as i32, antenna.1.cmp(&other.1));

                let antinode_is = match i_cmp {
                    std::cmp::Ordering::Less => [antenna.0 - i_diff, other.0 + i_diff],
                    std::cmp::Ordering::Greater => [antenna.0 + i_diff, other.0 - i_diff],
                    std::cmp::Ordering::Equal => [antenna.0, other.0],
                };

                let antinode_js = match j_cmp {
                    std::cmp::Ordering::Less => [antenna.1 - j_diff, other.1 + j_diff],
                    std::cmp::Ordering::Greater => [antenna.1 + j_diff, other.1 - j_diff],
                    std::cmp::Ordering::Equal => [antenna.1, other.1],
                };

                for i in 0..2 {
                    let proposed = (antinode_is[i], antinode_js[i]);

                    if is_in_bounds(input.0, proposed) {
                        antinodes.insert(proposed);
                    }
                }
            })
        });
    });

    antinodes.len()
}

#[allow(clippy::type_complexity)]
fn part2(input: &((i32, i32), HashMap<char, HashSet<(i32, i32)>>)) -> impl ToString {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    input.1.values().for_each(|antennae| {
        antennae.iter().for_each(|antenna| {
            antennae.iter().for_each(|other| {
                if antenna == other {
                    return;
                }

                let i_diff = antenna.0 - other.0;
                let j_diff = antenna.1 - other.1;

                let mut proposed = (antenna.0 - i_diff, antenna.1 - j_diff);
                while is_in_bounds(input.0, proposed) {
                    proposed = (proposed.0 - i_diff, proposed.1 - j_diff);
                }
                loop {
                    proposed = (proposed.0 + i_diff, proposed.1 + j_diff);
                    if !is_in_bounds(input.0, proposed) {
                        break;
                    }
                    antinodes.insert(proposed);
                }
            })
        });
    });

    antinodes.len()
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
                    ............\n\
                    ........0...\n\
                    .....0......\n\
                    .......0....\n\
                    ....0.......\n\
                    ......A.....\n\
                    ............\n\
                    ............\n\
                    ........A...\n\
                    .........A..\n\
                    ............\n\
                    ............\n\
                "
            ))
            .to_string(),
            "14"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&process(
                "
                    ............\n\
                    ........0...\n\
                    .....0......\n\
                    .......0....\n\
                    ....0.......\n\
                    ......A.....\n\
                    ............\n\
                    ............\n\
                    ........A...\n\
                    .........A..\n\
                    ............\n\
                    ............\n\
                "
            ))
            .to_string(),
            "34"
        );
    }
}
