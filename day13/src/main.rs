const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn with_adjusted_prize(&self, distance: i64) -> Self {
        Self {
            a: self.a,
            b: self.b,
            prize: (self.prize.0 + distance, self.prize.1 + distance),
        }
    }

    fn solve(&self) -> Option<(i64, i64)> {
        let b_left = self.prize.1 * self.a.0 - self.a.1 * self.prize.0;
        let bs = self.a.0 * self.b.1 - self.a.1 * self.b.0;
        let (b, b_rem) = (b_left / bs, b_left % bs);

        if b_rem != 0 {
            return None;
        }

        let a_left = self.prize.1 - b * self.b.1;
        let (a, a_rem) = (a_left / self.a.1, a_left % self.a.1);

        if a_rem != 0 {
            return None;
        }

        Some((a, b))
    }
}

fn process(input: &str) -> Vec<Machine> {
    input
        .trim()
        .split("\n\n")
        .map(|raw| {
            let parts = raw
                .splitn(3, "\n")
                .map(|line| {
                    let (x_raw, y_raw) = line.split_once(": ").unwrap().1.split_once(", ").unwrap();

                    (x_raw[2..].parse().unwrap(), y_raw[2..].parse().unwrap())
                })
                .collect::<Vec<_>>();

            Machine {
                a: parts[0],
                b: parts[1],
                prize: parts[2],
            }
        })
        .collect()
}

#[allow(clippy::ptr_arg)]
fn part1(input: &Vec<Machine>) -> impl ToString {
    input.iter().fold(0, |acc, &machine| {
        if let Some(solution) = machine.solve() {
            acc + solution.0 * 3 + solution.1
        } else {
            acc
        }
    })
}

#[allow(clippy::ptr_arg)]
fn part2(input: &Vec<Machine>) -> impl ToString {
    input.iter().fold(0, |acc, &machine| {
        if let Some(solution) = machine.with_adjusted_prize(10000000000000).solve() {
            acc + solution.0 * 3 + solution.1
        } else {
            acc
        }
    })
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
                    Button A: X+94, Y+34\n\
                    Button B: X+22, Y+67\n\
                    Prize: X=8400, Y=5400\n\
                    \n\
                    Button A: X+26, Y+66\n\
                    Button B: X+67, Y+21\n\
                    Prize: X=12748, Y=12176\n\
                    \n\
                    Button A: X+17, Y+86\n\
                    Button B: X+84, Y+37\n\
                    Prize: X=7870, Y=6450\n\
                    \n\
                    Button A: X+69, Y+23\n\
                    Button B: X+27, Y+71\n\
                    Prize: X=18641, Y=10279\n\
                "
            ))
            .to_string(),
            "480"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&process(
                "
                    Button A: X+94, Y+34\n\
                    Button B: X+22, Y+67\n\
                    Prize: X=8400, Y=5400\n\
                    \n\
                    Button A: X+26, Y+66\n\
                    Button B: X+67, Y+21\n\
                    Prize: X=12748, Y=12176\n\
                    \n\
                    Button A: X+17, Y+86\n\
                    Button B: X+84, Y+37\n\
                    Prize: X=7870, Y=6450\n\
                    \n\
                    Button A: X+69, Y+23\n\
                    Button B: X+27, Y+71\n\
                    Prize: X=18641, Y=10279\n\
                "
            ))
            .to_string(),
            "875318608908"
        );
    }
}
