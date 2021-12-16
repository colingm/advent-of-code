use aoc2021::{get_input, parse_u8_vec};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal { return Some(cost); }

        // Important as we may have already found a better way
        if cost > dist[position] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn convert_to_adj_list(rows: &Vec<Vec<u8>>, tiles: usize) -> Vec<Vec<Edge>> {
    let y_max = rows.len() * tiles;
    let x_max = rows[0].len() * tiles;
    let vertices = x_max * y_max;

    let mut adj_list: Vec<Vec<Edge>> = Vec::new();
    for y in 0..y_max {
        for x in 0..x_max {
            let mut edges: Vec<Edge> = Vec::new();
            for (i, j) in DIRECTIONS {
                let new_x: isize = x as isize + i as isize;
                let new_y: isize = y as isize + j as isize;
                let next: usize = (new_x + new_y * x_max as isize) as usize;
                if new_x >= 0 && (new_x as usize) < x_max && new_y >= 0 && (new_y as usize) < y_max {
                    let risk_x: usize = (new_x as usize) % (x_max / tiles);
                    let risk_y: usize = (new_y as usize) % (y_max / tiles);
                    let tile_x: usize = (new_x as usize) / (x_max / tiles);
                    let tile_y: usize = (new_y as usize) / (y_max / tiles);
                    let mut risk: usize = (rows[risk_y][risk_x] as usize) + tile_x + tile_y;
                    if risk > 9 {
                        risk -= 9;
                    }
                    edges.push(Edge { node: next, cost: risk });
                }
            }
            adj_list.push(edges);
        }
    }

    adj_list
}

fn part_1(rows: &Vec<Vec<u8>>) -> usize {
    let adj_list: Vec<Vec<Edge>> = convert_to_adj_list(rows, 1);

    shortest_path(&adj_list, 0, adj_list.len() - 1).unwrap()
}

fn part_2(rows: &Vec<Vec<u8>>) -> usize {
    let adj_list: Vec<Vec<Edge>> = convert_to_adj_list(rows, 5);

    shortest_path(&adj_list, 0, adj_list.len() - 1).unwrap()
}

fn main() {
    let input: Vec<Vec<u8>> = get_input("inputs/day_15.txt", &parse_u8_vec);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
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

    #[test]
    fn test_part_2_answer() {
        let input = get_input("inputs/day_15.txt", &parse_u8_vec);

        assert_eq!(part_2(&input), 2876);
    }
}