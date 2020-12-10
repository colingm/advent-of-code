use std::fs;

fn get_input(filename: &str) -> Vec<usize> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn part_1(original: &[usize]) -> usize {
    let mut adapters = original.to_vec();
    adapters.sort();

    let mut differences: Vec<usize> = (0..3).map(|_| 0).collect();
    let mut current_value: usize = 0;

    adapters.iter()
        .for_each(|value| {
            let difference = value - current_value;
            differences[difference - 1] += 1;
            current_value = *value;
        });

    differences[0] * (differences[2] + 1)
}

fn part_2(original: &[usize]) -> usize {
    let mut adapters = original.to_vec();
    adapters.sort();


    adapters.insert(0, 0);
    let max_jolts = *adapters.last().unwrap() + 3;
    adapters.push(max_jolts);

    let mut combinations: Vec<usize> = vec![1];
    (0..max_jolts).for_each(|_| combinations.push(0));

    adapters.iter()
        .for_each(|jolts| {
            for i in 1..=3 {
                if *jolts < i { continue; }
                combinations[*jolts] += combinations[jolts - i];
            }
        });

    *combinations.last().unwrap()
}

fn main() {
    let adapters = get_input("input.txt");

    println!("Part 1 Results: {}", part_1(&adapters));
    println!("Part 2 Results: {}", part_2(&adapters));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let adapters = get_input("test.txt");

        assert_eq!(part_1(&adapters), 220);
    }

    #[test]
    fn test_part_2() {
        let adapters = get_input("test.txt");

        assert_eq!(part_2(&adapters), 19208);
    }
}
