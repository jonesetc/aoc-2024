use std::iter::repeat_n;

const INPUT: &str = include_str!("../input.txt");

fn process(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .trim()
        .char_indices()
        .fold((vec![], vec![]), |(mut files, mut spaces), (i, c)| {
            if i % 2 == 0 {
                files.push(c.to_digit(10).unwrap() as usize);
            } else {
                spaces.push(c.to_digit(10).unwrap() as usize);
            };

            (files, spaces)
        })
}

fn part1(input: &(Vec<usize>, Vec<usize>)) -> impl ToString {
    let expanded: Vec<Option<usize>> = (0..input.0.len())
        .flat_map(|i| {
            let mut elements = vec![repeat_n(Some(i), input.0[i])];
            if let Some(&space) = input.1.get(i) {
                elements.push(repeat_n(None, space));
            };

            elements
        })
        .flatten()
        .collect();

    let mut range = 0..(expanded.len());

    let mut total = 0;
    while let Some(i) = range.next() {
        let block = expanded[i];
        if let Some(file) = block {
            total += i * file;
        } else if let Some(back_i) = range.rfind(|&back_i| expanded[back_i].is_some()) {
            total += i * expanded[back_i].unwrap();
        }
    }

    total
}

fn part2(input: &(Vec<usize>, Vec<usize>)) -> impl ToString {
    let mut files_expanded: Vec<(usize, Vec<usize>)> = input
        .0
        .iter()
        .enumerate()
        .map(|(i, &file_len)| (0, repeat_n(i, file_len).collect()))
        .collect();

    let mut spaces_expanded: Vec<(usize, Vec<usize>)> = input
        .1
        .iter()
        .map(|&space_len| (space_len, vec![]))
        .collect();

    for file_index in (0..(files_expanded.len())).rev() {
        for (space_index, space) in spaces_expanded.iter_mut().enumerate() {
            if space_index < file_index && space.0 >= files_expanded[file_index].1.len() {
                files_expanded[file_index].0 = files_expanded[file_index].1.len();
                space.0 -= files_expanded[file_index].0;

                space.1.append(&mut files_expanded[file_index].1);
                break;
            }
        }
    }

    let mut total = 0usize;
    let mut i = 0usize;

    for (sub_i, (num_empty_file_blocks, file_blocks)) in files_expanded.iter().enumerate() {
        for &block in file_blocks.iter() {
            total += block * i;
            i += 1;
        }
        i += *num_empty_file_blocks;

        if let Some((num_empty_space_blocks, space_blocks)) = spaces_expanded.get(sub_i) {
            for &block in space_blocks.iter() {
                total += block * i;
                i += 1;
            }
            i += *num_empty_space_blocks;
        }
    }

    total
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
        assert_eq!(part1(&process("2333133121414131402")).to_string(), "1928");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&process("2333133121414131402")).to_string(), "2858");
    }
}
