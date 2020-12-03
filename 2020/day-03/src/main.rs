use std::fs;

fn get_mountain(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mountain: Vec<String> = contents
        .lines()
        .map(ToOwned::to_owned)
        .collect();

    mountain
}

fn count_trees(mountain: &Vec<String>, rise: usize, run: usize) -> usize {
    let mut j: usize = 0;

    mountain.iter().step_by(rise)
        .filter(|row| {
            let width = row.chars().count();
            let object = row.chars().nth(j % width).unwrap();

            j += run;

            object == '#'
        })
        .count()
}

fn check_slopes(mountain: &Vec<String>) -> u64 {
    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    slopes.iter()
        .fold(1, |acc, slope| acc * (count_trees(mountain, slope.0, slope.1) as u64))
}

fn main() {
    let mountain = get_mountain("input.txt");

    println!("Part 1 Result: {}", count_trees(&mountain, 1, 3));
    println!("Part 2 Result: {}", check_slopes(&mountain));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_slope() {
        let mountain = get_mountain("test.txt");

        assert_eq!(count_trees(&mountain, 1, 3), 7);
    }

    #[test]
    fn test_multiple_steps() {
        let mountain = get_mountain("test.txt");

        assert_eq!(check_slopes(&mountain), 336);
    }
}
