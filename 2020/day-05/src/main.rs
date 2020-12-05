use std::fs;

#[derive(Debug)]
struct BoardingPass {
    pass_info: String,
    row: usize,
    column: usize,
    id: usize
}

impl BoardingPass {
    pub fn new(pass: &str) -> Self {
        let row = BoardingPass::get_row(pass);
        let column = BoardingPass::get_column(pass);
        let id = BoardingPass::get_id(&row, &column);

        BoardingPass {
            pass_info: pass.to_string(),
            row: row,
            column: column,
            id: id
        }
    }

    fn binary_find(pass: &str, first: usize, last: usize) -> usize {
        let characters: usize = last - first + 1;
        let base: usize = 2;
        let mut left: usize = 0;
        let mut right: usize = base.pow(characters as u32) - 1;

        for i in first..last {
            let diff = right - left;
            match pass.chars().nth(i).unwrap() {
                'F' | 'L' => {
                    right = right - ((diff + 1) / 2);
                },
                'B' | 'R' => {
                    left = right - (diff / 2);
                },
                _ => ()
            }
        }

        match pass.chars().nth(last).unwrap() {
            'F' | 'L' => left,
            'B' | 'R' => right,
            _ => 0
        }
    }

    fn get_row(pass: &str) -> usize {
        BoardingPass::binary_find(pass, 0, 6)
    }

    fn get_column(pass: &str) -> usize {
        BoardingPass::binary_find(pass, 7, 9)
    }

    fn get_id(row: &usize, column: &usize) -> usize {
        row * 8 + column
    }
}

fn find_max_id(boarding_passes: &Vec<BoardingPass>) -> usize {
    boarding_passes.iter()
        .max_by_key(|bp| bp.id)
        .unwrap()
        .id
}

fn find_seat(mut boarding_passes: Vec<BoardingPass>) -> usize {
    boarding_passes.sort_by(|a, b| a.id.cmp(&b.id));

    for (i, pass) in boarding_passes.iter().enumerate() {
        if boarding_passes[i + 1].id != (pass.id + 1) {
            return pass.id + 1
        }
    }

    0
}

fn get_boarding_passes(filename: &str) -> Vec<BoardingPass> {
    let contents = fs::read_to_string(filename)
        .expect("Unable to read file");

    contents.lines()
        .map(BoardingPass::new)
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
    fn test_construction() {
        let test_string = "FBFBBFFRLR";

        let boarding_pass = BoardingPass::new(test_string);

        assert_eq!(boarding_pass.row, 44);
        assert_eq!(boarding_pass.column, 5);
        assert_eq!(boarding_pass.id, 357);
    }

    #[test]
    fn test_construction1() {
        let test_string = "BFFFBBFRRR";

        let boarding_pass = BoardingPass::new(test_string);

        assert_eq!(boarding_pass.row, 70);
        assert_eq!(boarding_pass.column, 7);
        assert_eq!(boarding_pass.id, 567);
    }

    #[test]
    fn test_construction2() {
        let test_string = "FFFBBBFRRR";

        let boarding_pass = BoardingPass::new(test_string);

        assert_eq!(boarding_pass.row, 14);
        assert_eq!(boarding_pass.column, 7);
        assert_eq!(boarding_pass.id, 119);
    }

    #[test]
    fn test_construction3() {
        let test_string = "BBFFBBFRLL";

        let boarding_pass = BoardingPass::new(test_string);

        assert_eq!(boarding_pass.row, 102);
        assert_eq!(boarding_pass.column, 4);
        assert_eq!(boarding_pass.id, 820);
    }

    #[test]
    fn test_max_id() {
        let boarding_passes = get_boarding_passes("test.txt");

        assert_eq!(find_max_id(&boarding_passes), 820);
    }
}
