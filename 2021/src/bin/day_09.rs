use aoc2021::{get_input, parse_u8_vec};
use std::collections::HashSet;

const DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Coordinate {
    x: usize,
    y: usize
}

fn find_lowpoints(rows: &Vec<Vec<u8>>) -> Vec<Coordinate> {
    let row_count: isize = rows.len() as isize;
    let column_count: isize = rows[0].len() as isize;
    let mut lowpoints: Vec<Coordinate> = Vec::new();

    for (y, row) in rows.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let mut adjacent_cells: u8 = 0;
            let mut cells_less_than: u8 = 0;
            for (i, j) in DIRECTIONS.iter() {
                let new_x: isize = x as isize + *i as isize;
                let new_y: isize = y as isize + *j as isize;
                if new_x >= 0 && new_x < column_count && new_y >= 0 && new_y < row_count {
                    adjacent_cells += 1;
                    if cell < &rows[new_y as usize][new_x as usize] {
                        cells_less_than += 1;
                    }
                }
            }

            if adjacent_cells == cells_less_than {
                lowpoints.push(Coordinate { x, y });
            }
        }
    }

    lowpoints
}

fn find_basin_size(current: &Coordinate, rows: &Vec<Vec<u8>>, visited: &mut HashSet<Coordinate>) -> usize {
    if rows[current.y][current.x] == 9 { return 0; }

    let row_count: isize = rows.len() as isize;
    let column_count: isize = rows[0].len() as isize;
    let mut basin_size = 1;
    for (i, j) in DIRECTIONS.iter() {
        let new_x: isize = current.x as isize + *i as isize;
        let new_y: isize = current.y as isize + *j as isize;
        let next = Coordinate { x: new_x as usize, y: new_y as usize };
        if new_x >= 0 && new_x < column_count && new_y >= 0 && new_y < row_count && !visited.contains(&next) {
            visited.insert(next);
            basin_size += find_basin_size(&next, rows, visited);
        }
    }

    basin_size
}

fn part_1(rows: &Vec<Vec<u8>>) -> usize {
    let lowpoints: Vec<Coordinate> = find_lowpoints(rows);

    lowpoints.iter()
        .map(|c| rows[c.y][c.x] as usize + 1)
        .sum()
}

fn part_2(rows: &Vec<Vec<u8>>) -> usize {
    let lowpoints: Vec<Coordinate> = find_lowpoints(rows);
    let mut basins: Vec<usize> = Vec::new();

    for lowpoint in lowpoints.iter() {
        let mut visited: HashSet<Coordinate> = HashSet::new();
        visited.insert(*lowpoint);

        basins.push(find_basin_size(&lowpoint, rows, &mut visited));
    }

    basins.sort();
    let highest = basins.split_off(basins.len() - 3);
    highest.iter().fold(1, |a, b| a * b)
}

fn main() {
    let input: Vec<Vec<u8>> = get_input("inputs/day_09.txt", &parse_u8_vec);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_input("inputs/test/day_09.txt", &parse_u8_vec);

        assert_eq!(part_1(&input), 15);
    }

    #[test]
    fn test_part_1_answer() {
        let input = get_input("inputs/day_09.txt", &parse_u8_vec);

        assert_eq!(part_1(&input), 631);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("inputs/test/day_09.txt", &parse_u8_vec);

        assert_eq!(part_2(&input), 1134);
    }

    #[test]
    fn test_part_2_answer() {
        let input = get_input("inputs/day_09.txt", &parse_u8_vec);

        assert_eq!(part_2(&input), 821560);
    }
}