use std::iter::repeat_n;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: (usize, usize),
    vel: (i32, i32),
}

impl Robot {
    fn after_seconds(&self, seconds: usize, size: (usize, usize)) -> Self {
        Self {
            pos: (
                (self.pos.0 as i32 + (self.vel.0 * seconds as i32)).rem_euclid(size.0 as i32)
                    as usize,
                (self.pos.1 as i32 + (self.vel.1 * seconds as i32)).rem_euclid(size.1 as i32)
                    as usize,
            ),
            vel: self.vel,
        }
    }
}

fn process(input: &str) -> Vec<Robot> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let (pos_raw, vel_raw) = line.split_once(" ").unwrap();

            let pos_parts = pos_raw[2..].split_once(",").unwrap();
            let vel_parts = vel_raw[2..].split_once(",").unwrap();

            Robot {
                pos: (pos_parts.0.parse().unwrap(), pos_parts.1.parse().unwrap()),
                vel: (vel_parts.0.parse().unwrap(), vel_parts.1.parse().unwrap()),
            }
        })
        .collect()
}

#[allow(clippy::ptr_arg)]
fn part1(input: &Vec<Robot>, size: (usize, usize)) -> impl ToString {
    input
        .iter()
        .map(|robot| robot.after_seconds(100, size))
        .fold([0, 0, 0, 0], |mut acc, robot| {
            match (
                robot.pos.0.cmp(&(size.0 / 2)),
                robot.pos.1.cmp(&(size.1 / 2)),
            ) {
                (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => acc[0] += 1,
                (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => acc[1] += 1,
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => acc[2] += 1,
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => acc[3] += 1,
                _ => (),
            };
            acc
        })
        .iter()
        .product::<usize>()
}

#[allow(clippy::ptr_arg)]
fn part2(input: &Vec<Robot>, size: (usize, usize)) -> impl ToString {
    (0..)
        .map(|i| {
            (
                i,
                input
                    .iter()
                    .map(|robot| robot.after_seconds(i, size))
                    .collect::<Vec<Robot>>(),
            )
        })
        .map(|(i, robots)| {
            let mut display: Vec<Vec<bool>> =
                repeat_n(repeat_n(false, size.0).collect(), size.1).collect();

            robots
                .iter()
                .for_each(|robot| display[robot.pos.1][robot.pos.0] = true);

            let image = display
                .iter()
                .map(|line| {
                    let mut display_line = line
                        .iter()
                        .map(|&is_on| if is_on { '*' } else { ' ' })
                        .collect::<String>();
                    display_line.push('\n');

                    display_line
                })
                .collect::<String>();

            if image.contains("**********") {
                Some(i)
            } else {
                None
            }
        })
        .find(Option::is_some)
        .unwrap()
        .unwrap()
}

fn main() {
    println!("Processing input");
    let input = process(INPUT);
    println!("------");

    println!("Running part 1");
    println!("Result: {}", part1(&input, (101, 103)).to_string());
    println!("------");

    println!("Running part 2");
    println!("Result: {}", part2(&input, (101, 103)).to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                &process(
                    "
                    p=0,4 v=3,-3\n\
                    p=6,3 v=-1,-3\n\
                    p=10,3 v=-1,2\n\
                    p=2,0 v=2,-1\n\
                    p=0,0 v=1,3\n\
                    p=3,0 v=-2,-2\n\
                    p=7,6 v=-1,-3\n\
                    p=3,0 v=-1,-2\n\
                    p=9,3 v=2,3\n\
                    p=7,3 v=-1,2\n\
                    p=2,4 v=2,-3\n\
                    p=9,5 v=-3,-3\n\
                "
                ),
                (11, 7)
            )
            .to_string(),
            "12"
        );
    }
}
