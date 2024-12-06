use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn process(input: &str) -> String {
    input.trim().to_owned()
}

fn part1(input: &str) -> impl ToString {
    Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")
        .unwrap()
        .captures_iter(input)
        .map(|captures| captures[1].parse::<u32>().unwrap() * captures[2].parse::<u32>().unwrap())
        .sum::<u32>()
}

fn part2(input: &str) -> impl ToString {
    let mut is_enabled = true;
    Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|don't\(\)|do\(\)")
        .unwrap()
        .captures_iter(input)
        .filter_map(|captures| {
            match &captures[0] {
                "do()" => is_enabled = true,
                "don't()" => is_enabled = false,
                _ => {
                    if is_enabled {
                        return Some(
                            captures[1].parse::<u32>().unwrap()
                                * captures[2].parse::<u32>().unwrap(),
                        );
                    };
                }
            };
            None
        })
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
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            ))
            .to_string(),
            "161"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&process(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ))
            .to_string(),
            "48"
        );
    }
}
