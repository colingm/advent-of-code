use aoc2021::get_input;

fn parse_characters(line: &str) -> Vec<u8> {
    line.chars()
        .map(|val| val.to_string().parse().unwrap())
        .collect()
}

fn part_1(lines: &Vec<Vec<u8>>) -> usize {
    let bit_count = lines[0].len();
    let mut bits: Vec<usize> = vec![0; bit_count];

    lines.iter()
        .for_each(|val| {
            val.iter()
                .enumerate()
                .for_each(|(i, bit)| {
                    if *bit == 1 {
                        bits[i] += 1;
                    }
                });
        });

    let half: usize = (lines.len() + 1) / 2;

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;

    bits.iter()
        .for_each(|val| {
            gamma <<= 1;
            epsilon <<= 1;

            let mut bit = 0;
            if *val > half {
                bit = 1;
            }

            gamma += bit;
            epsilon += ((bit % 2) == 0) as usize;
        });

    println!("gamma: {}, epsilon: {}", gamma, epsilon);

    gamma * epsilon
}

fn find_rating(numbers: &Vec<Vec<u8>>, most_common: bool) -> usize {
    let bit_count = numbers[0].len();
    let mut current_bit = 0;
    let mut current: Vec<Vec<u8>> = numbers.to_vec();

    while {
        let mut ones: Vec<Vec<u8>> = Vec::new();
        let mut zeros: Vec<Vec<u8>> = Vec::new();
        let half: usize = (current.len() + 1) / 2;
        let mut one_count = 0;
        current.iter()
            .for_each(|val| {
                if val[current_bit] == 1 {
                    one_count += 1;
                    ones.push(val.to_vec())
                } else {
                    zeros.push(val.to_vec());
                }
            });

        if (most_common && one_count >= half) || (!most_common && one_count < half) {
            current = ones.to_vec();
        } else {
            current = zeros.to_vec();
        }

        current_bit += 1;

        current.len() > 1 && current_bit < bit_count
    } {}

    let bits: Vec<String> = current[0].iter().map(|val| val.to_string()).collect();

    usize::from_str_radix(bits.join("").as_str(), 2).unwrap()
}

fn part_2(lines: &Vec<Vec<u8>>) -> usize {
    let oxygen = find_rating(lines, true);
    let co2 = find_rating(lines, false);

    println!("oxygen: {}, co2: {}", oxygen, co2);

    oxygen * co2
}

fn main() {
    let input = get_input("inputs/day_03.txt", &parse_characters);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_input("inputs/test/day_03.txt", &parse_characters);

        println!("Input: {:?}", input);

        assert_eq!(part_1(&input), 198);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("inputs/test/day_03.txt", &parse_characters);

        assert_eq!(part_2(&input), 230);
    }
}