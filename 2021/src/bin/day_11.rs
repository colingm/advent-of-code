use aoc2021::{get_input, parse_u8_vec};

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize
}

const DIRECTIONS: [Point; 8] = [Point { x: -1, y: -1}, Point { x: -1, y: 0}, Point { x: -1, y: 1}, Point { x: 0, y: 1}, Point { x: 1, y: 1}, Point { x: 1, y: 0}, Point { x: 1, y: -1}, Point { x: 0, y: -1}];

fn increase_all(octopi: &mut Vec<Vec<u8>>) {
    for row in octopi {
        for octopus in row {
            *octopus += 1;
        }
    }
}

fn find_ready_to_flash(octopi: &Vec<Vec<u8>>) -> Vec<Point> {
    let mut need_to_flash: Vec<Point> = Vec::new();
    for (y, row) in octopi.iter().enumerate() {
        for (x, octopus) in row.iter().enumerate() {
            if *octopus > 9 {
                need_to_flash.push(Point { x: x as isize, y: y as isize });
            }
        }
    }

    need_to_flash
}

fn flash(octopi: &mut Vec<Vec<u8>>, center: &Point) {
    let rows: isize = octopi.len() as isize;
    let columns: isize = octopi[0].len() as isize;

    for dir in DIRECTIONS.iter() {
        let next: Point = Point { x: (center.x + dir.x), y: (center.y + dir.y) };
        if next.x >= 0 && next.x < columns && next.y >= 0 && next.y < rows {
            let octopus: &mut u8 = &mut octopi[next.y as usize][next.x as usize];
            if *octopus <= 9 {
                *octopus += 1;
                if *octopus > 9 {
                    flash(octopi, &next);
                }
            }
        }
    }
}

fn reset(octopi: &mut Vec<Vec<u8>>) -> usize {
    let mut flashes: usize = 0;
    for row in octopi {
        for octopus in row {
            if *octopus > 9 {
                *octopus = 0;
                flashes += 1;
            }
        }
    }

    flashes
}

fn part_1(rows: &Vec<Vec<u8>>) -> usize {
    let mut octopi: Vec<Vec<u8>> = rows.clone();
    let mut flashes: usize = 0;
    for _ in 0..100 {
        increase_all(&mut octopi);
        let need_to_flash: Vec<Point> = find_ready_to_flash(&octopi);
        for point in need_to_flash.iter() {
            flash(&mut octopi, &point);
        }

        flashes += reset(&mut octopi);
    }

    flashes
}

fn part_2(rows: &Vec<Vec<u8>>) -> usize {
    let mut octopi: Vec<Vec<u8>> = rows.clone();
    let total: usize = octopi.len() * octopi[0].len();

    for i in 0..500 {
        increase_all(&mut octopi);
        let need_to_flash: Vec<Point> = find_ready_to_flash(&octopi);
        for point in need_to_flash.iter() {
            flash(&mut octopi, &point);
        }

        let flashes: usize = reset(&mut octopi);

        if flashes == total {
            return i + 1;
        }
    }

    0
}

fn main() {
    let input: Vec<Vec<u8>> = get_input("inputs/day_11.txt", &parse_u8_vec);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_input("inputs/test/day_11.txt", &parse_u8_vec);

        assert_eq!(part_1(&input), 1656);
    }

    #[test]
    fn test_part_1_answer() {
        let input = get_input("inputs/day_11.txt", &parse_u8_vec);

        assert_eq!(part_1(&input), 1634);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("inputs/test/day_11.txt", &parse_u8_vec);

        assert_eq!(part_2(&input), 195);
    }

    #[test]
    fn test_part_2_answer() {
        let input = get_input("inputs/day_11.txt", &parse_u8_vec);

        assert_eq!(part_2(&input), 210);
    }
}