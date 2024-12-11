use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn process(input: &str) -> Vec<usize> {
    input
        .trim()
        .split_ascii_whitespace()
        .map(|raw| raw.parse().unwrap())
        .collect()
}

fn blink(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return vec![1];
    }

    let num_digits = stone.ilog10() + 1;
    if num_digits % 2 == 0 {
        let divisor = 10usize.pow(num_digits / 2);
        return vec![stone / divisor, stone % divisor];
    }

    vec![stone * 2024]
}

#[allow(clippy::ptr_arg)]
fn count_after_blinks(initial_stones: &Vec<usize>, num_blinks: usize) -> usize {
    let mut stone_counts: HashMap<usize, usize> =
        initial_stones
            .iter()
            .fold(HashMap::new(), |mut acc, &stone| {
                *acc.entry(stone).or_default() += 1;
                acc
            });

    for _ in 0..num_blinks {
        stone_counts = stone_counts
            .into_iter()
            .flat_map(|(stone, count)| {
                blink(stone)
                    .iter()
                    .map(|&new_stone| (new_stone, count))
                    .collect::<Vec<_>>()
            })
            .fold(HashMap::new(), |mut acc, (stone, count)| {
                *acc.entry(stone).or_default() += count;
                acc
            })
    }

    stone_counts.values().sum::<usize>()
}

fn part1(input: &Vec<usize>) -> impl ToString {
    count_after_blinks(input, 25)
}

fn part2(input: &Vec<usize>) -> impl ToString {
    count_after_blinks(input, 75)
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
        assert_eq!(part1(&process("125 17")).to_string(), "55312");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&process("125 17")).to_string(), "65601038650482");
    }
}
