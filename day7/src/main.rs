const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
    Cat,
}

fn process(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let (target, parts) = line.split_once(": ").unwrap();
            (
                target.parse().unwrap(),
                parts
                    .split_ascii_whitespace()
                    .map(|raw| raw.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[allow(clippy::ptr_arg)]
fn part1(input: &Vec<(u64, Vec<u64>)>) -> impl ToString {
    input
        .iter()
        .filter(|(target, values)| {
            let num_operations = (values.len() - 1) as u32;
            (0..(2u64.pow(num_operations)))
                .map(|int| {
                    (0..num_operations)
                        .map(|shifts| match (int >> shifts) & 1 {
                            0 => Op::Add,
                            1 => Op::Mul,
                            _ => unreachable!(),
                        })
                        .collect::<Vec<Op>>()
                })
                .any(|ops| {
                    values
                        .iter()
                        .enumerate()
                        .fold(None, |acc, (i, &value)| {
                            if let Some(total) = acc {
                                match ops[i - 1] {
                                    Op::Add => Some(total + value),
                                    Op::Mul => Some(total * value),
                                    _ => unreachable!(),
                                }
                            } else {
                                Some(value)
                            }
                        })
                        .unwrap()
                        == *target
                })
        })
        .map(|(target, _)| target)
        .sum::<u64>()
}

#[allow(clippy::ptr_arg)]
fn part2(input: &Vec<(u64, Vec<u64>)>) -> impl ToString {
    input
        .iter()
        .filter(|(target, values)| {
            let num_operations = (values.len() - 1) as u32;
            (0..(3u64.pow(num_operations)))
                .map(|int| {
                    (0..num_operations)
                        .map(|shifts| match (int / 3u64.pow(shifts)) % 3 {
                            0 => Op::Add,
                            1 => Op::Mul,
                            2 => Op::Cat,
                            _ => unreachable!(),
                        })
                        .collect::<Vec<Op>>()
                })
                .any(|ops| {
                    values
                        .iter()
                        .enumerate()
                        .fold(None, |acc, (i, &value)| {
                            if let Some(total) = acc {
                                match ops[i - 1] {
                                    Op::Add => Some(total + value),
                                    Op::Mul => Some(total * value),
                                    Op::Cat => Some(format!("{}{}", total, value).parse().unwrap()),
                                }
                            } else {
                                Some(value)
                            }
                        })
                        .unwrap()
                        == *target
                })
        })
        .map(|(target, _)| target)
        .sum::<u64>()
}

fn main() {
    println!("Processing input");
    let input = process(INPUT);
    println!("------");

    println!("Running part 1");
    println!("Result: {}", part1(&input).to_string());
    println!("------");

    // 4105723140243 too low
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
                    190: 10 19\n\
                    3267: 81 40 27\n\
                    83: 17 5\n\
                    156: 15 6\n\
                    7290: 6 8 6 15\n\
                    161011: 16 10 13\n\
                    192: 17 8 14\n\
                    21037: 9 7 18 13\n\
                    292: 11 6 16 20\n\
                "
            ))
            .to_string(),
            "3749"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&process(
                "
                    190: 10 19\n\
                    3267: 81 40 27\n\
                    83: 17 5\n\
                    156: 15 6\n\
                    7290: 6 8 6 15\n\
                    161011: 16 10 13\n\
                    192: 17 8 14\n\
                    21037: 9 7 18 13\n\
                    292: 11 6 16 20\n\
                "
            ))
            .to_string(),
            "11387"
        );
    }
}
