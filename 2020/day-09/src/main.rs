use std::{
    fs,
    cmp::Ordering,
};

fn get_input(filename: &str) -> Vec<usize> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn part_1(numbers: &[usize], preamble_size: usize) -> usize {
    'window: for batch in numbers.windows(preamble_size + 1).skip(1) {
        for (i, x) in batch[..(preamble_size-1)].iter().enumerate() {
            for y in batch[(i+1)..preamble_size].iter() {
                if x + y == batch[preamble_size] {
                    continue 'window;
                }
            }
        }

        return batch[preamble_size];
    }

    0
}

fn part_2(numbers: &[usize], preamble_size: usize) -> usize {
    let target = part_1(numbers, preamble_size);
    let mut start = 0;
    let mut end = 0;
    let mut sum = 0;

    loop {
        match sum.cmp(&target) {
            Ordering::Less => {
                sum += numbers[end];
                end += 1;
            },
            Ordering::Equal => break,
            Ordering::Greater => {
                sum -= numbers[start];
                start += 1;
            }

        }
    }

    let window = &numbers[start..end];
    let min = window.iter().min().unwrap();
    let max = window.iter().max().unwrap();

    min + max
}

fn main() {
    let numbers = get_input("input.txt");

    println!("Part 1 Results: {}", part_1(&numbers, 25));
    println!("Part 2 Results: {}", part_2(&numbers, 25));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let numbers = get_input("test.txt");

        assert_eq!(part_1(&numbers, 5), 127);
    }

    #[test]
    fn test_part_2() {
        let numbers = get_input("test.txt");

        assert_eq!(part_2(&numbers, 5), 62);
    }
}
