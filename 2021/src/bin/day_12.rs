use aoc2021::{get_input, parse_string};
use std::collections::HashMap;

const START: &str = "start";
const END: &str = "end";

fn is_small_cave(cave: &String) -> bool {
    *cave == cave.to_lowercase()
}

fn build_tunnels(rows: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut paths: HashMap<String, Vec<String>> = HashMap::new();
    for row in rows {
        let parts: Vec<String> = row.split("-").map(|s| s.to_string()).collect();
        paths.entry(parts[0].to_string()).or_insert(Vec::new()).push(parts[1].to_string());
        paths.entry(parts[1].to_string()).or_insert(Vec::new()).push(parts[0].to_string());
    }

    paths
}

fn get_paths(tunnels: &HashMap<String, Vec<String>>, cave: &String, path: &String, has_double_visited: bool) -> usize {
    let next_tunnels: &Vec<String> = tunnels.get(cave).unwrap();
    let mut tunnel_count: usize = 0;

    for tunnel_end in next_tunnels {
        if tunnel_end == END {
            tunnel_count += 1;
        } else if !is_small_cave(tunnel_end) || !path.contains(tunnel_end) || (!has_double_visited && tunnel_end != START) {
            let mut visited = has_double_visited;
            if is_small_cave(tunnel_end) && path.contains(tunnel_end) {
                visited = true;
            }
            let next_path = path.clone() + "," + tunnel_end;
            tunnel_count += get_paths(tunnels, tunnel_end, &next_path, visited);
        }
    }

    tunnel_count
}

fn part_1(rows: &Vec<String>) -> usize {
    let tunnels: HashMap<String, Vec<String>> = build_tunnels(rows);
    let path: String = START.to_string();
    get_paths(&tunnels, &path, &path, true)
}

fn part_2(rows: &Vec<String>) -> usize {
    let tunnels: HashMap<String, Vec<String>> = build_tunnels(rows);
    let path: String = START.to_string();
    get_paths(&tunnels, &path, &path, false)
}

fn main() {
    let input: Vec<String> = get_input("inputs/day_12.txt", &parse_string);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_easy() {
        let input = get_input("inputs/test/day_12a.txt", &parse_string);

        assert_eq!(part_1(&input), 10);
    }

    #[test]
    fn test_part_1_medium() {
        let input = get_input("inputs/test/day_12b.txt", &parse_string);

        assert_eq!(part_1(&input), 19);
    }

    #[test]
    fn test_part_1_hard() {
        let input = get_input("inputs/test/day_12c.txt", &parse_string);

        assert_eq!(part_1(&input), 226);
    }

    #[test]
    fn test_part_1_answer() {
        let input = get_input("inputs/day_12.txt", &parse_string);

        assert_eq!(part_1(&input), 4691);
    }

    #[test]
    fn test_part_2_easy() {
        let input = get_input("inputs/test/day_12a.txt", &parse_string);

        assert_eq!(part_2(&input), 36);
    }

    #[test]
    fn test_part_2_medium() {
        let input = get_input("inputs/test/day_12b.txt", &parse_string);

        assert_eq!(part_2(&input), 103);
    }

    #[test]
    fn test_part_2_hard() {
        let input = get_input("inputs/test/day_12c.txt", &parse_string);

        assert_eq!(part_2(&input), 3509);
    }

    #[test]
    fn test_part_2_answer() {
        let input = get_input("inputs/day_12.txt", &parse_string);

        assert_eq!(part_2(&input), 140718);
    }
}