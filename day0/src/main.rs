const INPUT: &str = include_str!("../input.txt");

fn process(input: &str) -> String {
    input.trim().to_owned()
}

fn part1(input: &str) -> impl ToString {
    input.to_owned()
}

fn part2(input: &str) -> impl ToString {
    input.chars().rev().collect::<String>()
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
        assert_eq!(part1(&process("foo")).to_string(), "foo");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&process("foo")).to_string(), "oof");
    }
}
