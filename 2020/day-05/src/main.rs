use std::fs;

fn get_pass_id(pass: &str) -> usize {
    let bin_index: String = pass.chars()
        .map(|c| {
            match c {
                'B' | 'R' => '1',
                _ => '0'
            }
        })
        .collect();
    let pass_num: usize = usize::from_str_radix(bin_index.as_str(), 2).unwrap();

    let row = pass_num >> 3;
    let column = pass_num & 7;

    row * 8 + column
}

fn find_max_id(boarding_passes: &Vec<usize>) -> usize {
    boarding_passes.iter()
        .max()
        .cloned()
        .unwrap()
}

fn find_seat(mut boarding_passes: Vec<usize>) -> usize {
    boarding_passes.sort();

    for (i, pass) in boarding_passes.iter().enumerate() {
        if boarding_passes[i + 1] != (pass + 1) {
            return pass + 1
        }
    }

    0
}

fn get_boarding_passes(filename: &str) -> Vec<usize> {
    let contents = fs::read_to_string(filename)
        .expect("Unable to read file");

    contents.lines()
        .map(get_pass_id)
        .collect()
}

fn main() {
    let boarding_passes = get_boarding_passes("input.txt");

    println!("Part 1 Results: {}", find_max_id(&boarding_passes));
    println!("Part 2 Results: {}", find_seat(boarding_passes))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_pass_id() {
        let mut test_string = "FBFBBFFRLR";
        assert_eq!(get_pass_id(test_string), 357);
        test_string = "BFFFBBFRRR";
        assert_eq!(get_pass_id(test_string), 567);
        test_string = "FFFBBBFRRR";
        assert_eq!(get_pass_id(test_string), 119);
        test_string = "BBFFBBFRLL";
        assert_eq!(get_pass_id(test_string), 820);
    }

    #[test]
    fn test_max_id() {
        let boarding_passes = get_boarding_passes("test.txt");

        assert_eq!(find_max_id(&boarding_passes), 820);
    }
}
