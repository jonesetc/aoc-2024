use std::collections::HashMap;
use std::iter::zip;

const INPUT: &str = include_str!("../input.txt");

fn process(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    input.trim().split("\n").for_each(|line| {
        let parts = line
            .split_ascii_whitespace()
            .map(|raw| raw.parse().unwrap())
            .collect::<Vec<u32>>();

        left.push(parts[0]);
        right.push(parts[1]);
    });

    (left, right)
}

fn part1(input: (Vec<u32>, Vec<u32>)) -> impl ToString {
    let (mut left, mut right) = input;
    left.sort();
    right.sort();

    zip(left, right).map(|(a, b)| a.abs_diff(b)).sum::<u32>()
}

fn part2(input: (Vec<u32>, Vec<u32>)) -> impl ToString {
    let (left, right) = input;

    let right_counts = right.iter().fold(HashMap::new(), |mut acc, curr| {
        *acc.entry(curr).or_insert(0) += 1;
        acc
    });

    left.iter()
        .map(|curr| curr * right_counts.get(curr).unwrap_or(&0))
        .sum::<u32>()
}

fn main() {
    println!("Processing input");
    let input = process(INPUT);
    println!("------");

    println!("Running part 1");
    println!("Result: {}", part1(input.clone()).to_string());
    println!("------");

    println!("Running part 2");
    println!("Result: {}", part2(input.clone()).to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1((vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])).to_string(),
            "11"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2((vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])).to_string(),
            "31"
        );
    }
}
