use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn process(input: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let (dep_lines, update_lines) = input.trim().split_once("\n\n").unwrap();

    let deps = dep_lines.split("\n").fold(
        HashMap::new(),
        |mut acc: HashMap<u32, HashSet<u32>>, line| {
            let (dep, target) = line.split_once("|").unwrap();
            acc.entry(dep.parse().unwrap())
                .or_default()
                .insert(target.parse().unwrap());

            acc
        },
    );

    let updates = update_lines
        .split("\n")
        .map(|line| line.split(",").map(|raw| raw.parse().unwrap()).collect())
        .collect();

    (deps, updates)
}

fn part1(input: &(HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>)) -> impl ToString {
    input
        .1
        .iter()
        .filter(|update| {
            let mut seen: HashSet<u32> = HashSet::new();
            for item in update.iter() {
                if let Some(dependants) = input.0.get(item) {
                    if dependants.intersection(&seen).count() > 0 {
                        return false;
                    }
                }
                seen.insert(*item);
            }
            true
        })
        .map(|update| update[update.len() / 2])
        .sum::<u32>()
}

fn part2(input: &(HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>)) -> impl ToString {
    input
        .1
        .iter()
        .filter(|update| {
            let mut seen: HashSet<u32> = HashSet::new();
            for item in update.iter() {
                if let Some(dependants) = input.0.get(item) {
                    if dependants.intersection(&seen).count() > 0 {
                        return true;
                    }
                }
                seen.insert(*item);
            }
            false
        })
        .cloned()
        .map(|mut update| {
            update.sort_by(|a, b| {
                if input
                    .0
                    .get(a)
                    .map_or(false, |dependents| dependents.contains(b))
                {
                    Ordering::Less
                } else if input
                    .0
                    .get(b)
                    .map_or(false, |dependents| dependents.contains(a))
                {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            update
        })
        .map(|update| update[update.len() / 2])
        .sum::<u32>()
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
                    47|53\n\
                    97|13\n\
                    97|61\n\
                    97|47\n\
                    75|29\n\
                    61|13\n\
                    75|53\n\
                    29|13\n\
                    97|29\n\
                    53|29\n\
                    61|53\n\
                    97|53\n\
                    61|29\n\
                    47|13\n\
                    75|47\n\
                    97|75\n\
                    47|61\n\
                    75|61\n\
                    47|29\n\
                    75|13\n\
                    53|13\n\
                    \n\
                    75,47,61,53,29\n\
                    97,61,53,29,13\n\
                    75,29,13\n\
                    75,97,47,61,53\n\
                    61,13,29\n\
                    97,13,75,29,47\n\
                "
            ))
            .to_string(),
            "143",
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&process(
                "
                    47|53\n\
                    97|13\n\
                    97|61\n\
                    97|47\n\
                    75|29\n\
                    61|13\n\
                    75|53\n\
                    29|13\n\
                    97|29\n\
                    53|29\n\
                    61|53\n\
                    97|53\n\
                    61|29\n\
                    47|13\n\
                    75|47\n\
                    97|75\n\
                    47|61\n\
                    75|61\n\
                    47|29\n\
                    75|13\n\
                    53|13\n\
                    \n\
                    75,47,61,53,29\n\
                    97,61,53,29,13\n\
                    75,29,13\n\
                    75,97,47,61,53\n\
                    61,13,29\n\
                    97,13,75,29,47\n\
                "
            ))
            .to_string(),
            "123",
        );
    }
}
