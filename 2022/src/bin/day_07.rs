use aoc2022::{read_string, integer, string};
use pom::set::Set;
use pom::char_class::*;
use pom::parser::*;

#[derive(Debug)]
enum Command {
    CD(String),
    LS(Vec<Node>)
}

#[derive(Debug)]
enum Node {
    Directory,
    File(usize)
}

fn build_sizes(commands: &Vec<Command>) -> Vec<usize> {
    let mut directory_sizes: Vec<usize> = Vec::new();
    let mut current_subdirectories: Vec<usize> = Vec::new();
    commands.iter()
        .for_each(|c| {
            match c {
                Command::CD(name) => {
                    match name.to_str() {
                        ".." => { directory_sizes.push(current_subdirectories.pop().unwrap()); },
                        _ => { current_subdirectories.push(0); }
                    }
                },
                Command::LS(contents) => {
                    contents.iter()
                        .for_each(|n| {
                            match n {
                                Node::Directory => {},
                                Node::File(f) => {
                                    current_subdirectories.iter_mut().for_each(|x| *x += f )
                                }
                            }
                        });
                }
            }
        });
    directory_sizes.append(&mut current_subdirectories);

    directory_sizes
}

fn parse(input: &String) -> Vec<Command> {
    let directory: Parser<u8, Node> = (seq(b"dir") * sym(b' ') * is_a(alpha).repeat(1..).discard())
        .map(|_x| Node::Directory);
    let file: Parser<u8, Node> = (integer() - sym(b' ') - (is_a(alpha) | sym(b'.')).repeat(1..))
        .map(|x| Node::File(x));
    let change_command: Parser<u8, Command> = (seq(b"$ cd ") * string())
        .map(|x| Command::CD(x));
    let list_command: Parser<u8, Command> = (seq(b"$ ls\n") * list(directory | file, sym(b'\n')))
        .map(|x| Command::LS(x));
    let all_commands: Parser<u8, Vec<Command>> = list(change_command | list_command, sym(b'\n'));

    let result: Vec<Command> = all_commands.parse(input.as_bytes()).unwrap();

    result
}

fn part_1(input: &String) -> usize {
    let commands: Vec<Command> = parse(input);
    let sizes: Vec<usize> = build_sizes(&commands);

    sizes.iter()
        .filter(|s| **s <= 100000)
        .sum()
}

fn part_2(input: &String) -> usize {
    let commands: Vec<Command> = parse(input);
    let sizes: Vec<usize> = build_sizes(&commands);
    let used_space: usize = *sizes.iter().max().unwrap();
    let available_space: usize = 70000000 - used_space;
    let need_to_delete: usize = 30000000 - available_space;

    *sizes.iter()
        .filter(|s| **s >= need_to_delete)
        .min()
        .unwrap()
}

fn main() {
    let input = read_string("inputs/day_07.txt");

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_string("inputs/test/day_07.txt");

        assert_eq!(part_1(&input), 95437);
    }

    #[test]
    fn test_part_2() {
        let input = read_string("inputs/test/day_07.txt");

        assert_eq!(part_2(&input), 24933642);
    }
}