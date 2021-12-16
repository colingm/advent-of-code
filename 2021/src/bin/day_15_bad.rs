use aoc2021::{get_input, parse_u8_vec};

const DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

struct Graph {
    y_max: usize,
    x_max: usize,
    tile_count: usize,
    vertices: usize,
    risk_map: Vec<Vec<u8>>,
    distances: Vec<usize>,
    visited: Vec<bool>
}

impl Graph {
    fn new(rows: &Vec<Vec<u8>>, tile_count: usize) -> Self {
        let y_max = rows.len() * tile_count;
        let x_max = rows[0].len() * tile_count;
        let vertices = x_max * y_max;
        let mut distances = vec![usize::MAX; vertices];
        let visited = vec![false; vertices];
        distances[0] = 0;

        Self {
            y_max,
            x_max,
            tile_count,
            vertices,
            risk_map: rows.clone(),
            distances,
            visited
        }
    }

    fn min_distance(&self) -> usize {
        let mut min = usize::MAX;
        let mut min_index = 0;
        for i in 0..self.vertices {
            if !self.visited[i] && self.distances[i] < min {
                min = self.distances[i];
                min_index = i;
            }
        }

        min_index
    }

    fn build(&mut self) {
        for _ in 0..self.vertices {
            let min_index = self.min_distance();
            self.visited[min_index] = true;
            let x: usize = min_index % self.x_max;
            let y: usize = min_index / self.x_max;

            for (i, j) in DIRECTIONS {
                let new_x: isize = x as isize + i as isize;
                let new_y: isize = y as isize + j as isize;
                let next: usize = (new_x + new_y * self.x_max as isize) as usize;
                if new_x >= 0 && (new_x as usize) < self.x_max && new_y >= 0 && (new_y as usize) < self.y_max {
                    let risk_x: usize = (new_x as usize) % (self.x_max / self.tile_count);
                    let risk_y: usize = (new_y as usize) % (self.y_max / self.tile_count);
                    let tile_x: usize = (new_x as usize) / (self.x_max / self.tile_count);
                    let tile_y: usize = (new_y as usize) / (self.y_max / self.tile_count);
                    let mut risk: usize = (self.risk_map[risk_y][risk_x] as usize) + tile_x + tile_y;
                    if risk > 9 {
                        risk -= 9;
                    }
                    if !self.visited[next] && self.distances[min_index] + risk < self.distances[next] {
                        self.distances[next] = self.distances[min_index] + risk;
                    }
                }
            }
        }
    }

    fn lowest_risk(&self) -> usize {
        self.distances[self.vertices - 1]
    }
}

fn part_1(rows: &Vec<Vec<u8>>) -> usize {
    let mut risk_graph: Graph = Graph::new(rows, 1);

    risk_graph.build();

    risk_graph.lowest_risk()
}

fn part_2(rows: &Vec<Vec<u8>>) -> usize {
    let mut risk_graph: Graph = Graph::new(rows, 5);

    risk_graph.build();

    risk_graph.lowest_risk()
}

fn main() {
    let input: Vec<Vec<u8>> = get_input("inputs/day_15.txt", &parse_u8_vec);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input)); // This won't finish :(
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_input("inputs/test/day_15.txt", &parse_u8_vec);

        assert_eq!(part_1(&input), 40);
    }

    #[test]
    fn test_part_1_answer() {
        let input = get_input("inputs/day_15.txt", &parse_u8_vec);

        assert_eq!(part_1(&input), 523);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("inputs/test/day_15.txt", &parse_u8_vec);

        assert_eq!(part_2(&input), 315);
    }

    // #[test]
    // fn test_part_2_answer() {
    //     let input = get_input("inputs/day_15.txt", &parse_u8_vec);

    //     assert_eq!(part_2(&input), 2516901104210);
    // }
}