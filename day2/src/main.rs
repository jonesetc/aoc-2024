const INPUT: &str = include_str!("../input.txt");

fn process(input: &str) -> Vec<Vec<i8>> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|raw| raw.parse().unwrap())
                .collect()
        })
        .collect()
}

fn check_is_safe(report: &[i8]) -> bool {
    let mut prev = None;
    for levels in report.windows(2) {
        let diff = levels[0] - levels[1];
        if diff == 0 || diff.abs() > 3 {
            return false;
        }

        if prev.is_some_and(|prev_diff| prev_diff * diff < 0) {
            return false;
        }

        prev = Some(diff)
    }

    true
}

fn part1(input: Vec<Vec<i8>>) -> impl ToString {
    input
        .iter()
        .map(|report| check_is_safe(report))
        .filter(|&is_safe| is_safe)
        .count()
}

fn part2(input: Vec<Vec<i8>>) -> impl ToString {
    input
        .iter()
        .map(|report| {
            if check_is_safe(report) {
                return true;
            }

            (0..report.len()).any(|skipped| {
                let skipped_report = report
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &level)| if i == skipped { None } else { Some(level) })
                    .collect::<Vec<i8>>();
                check_is_safe(&skipped_report)
            })
        })
        .filter(|&is_safe| is_safe)
        .count()
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
            part1(vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ])
            .to_string(),
            "2"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ])
            .to_string(),
            "4"
        );
    }
}
