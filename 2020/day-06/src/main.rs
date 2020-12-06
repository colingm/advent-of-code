use std::fs;
use std::collections::HashMap;

struct Group {
    answers: HashMap<char, usize>,
    people: usize
}

impl Group {
    pub fn new(line: &str) -> Self {
        let mut answers: HashMap<char, usize> = HashMap::new();
        let mut people: usize = 0;
        line.split_whitespace()
            .for_each(|x| {
                people += 1;
                x.chars()
                    .for_each(|c| {
                        *answers.entry(c).or_insert(0) += 1;
                    });
            });

        Self {
            answers,
            people
        }
    }

    pub fn group_answers(&self) -> usize {
        self.answers.iter()
            .filter(|(_, count)| self.people == **count)
            .count()
    }
}

fn get_groups(filename: &str) -> Vec<Group> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents
        .split("\n\n")
        .map(Group::new)
        .collect()
}

fn get_total_answer_count(groups: &Vec<Group>) -> usize {
    groups.iter()
        .map(|g| g.answers.len())
        .sum()
}

fn get_matching_answer_count(groups: &Vec<Group>) -> usize {
    groups.iter()
        .map(|g| g.group_answers())
        .sum()
}

fn main() {
    let groups = get_groups("input.txt");

    println!("Part 1 Results: {}", get_total_answer_count(&groups));
    println!("Part 2 Results: {}", get_matching_answer_count(&groups));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answer_count() {
        let groups = get_groups("test.txt");

        assert_eq!(get_total_answer_count(&groups), 11)
    }

    #[test]
    fn test_matching_answers() {
        let groups = get_groups("test.txt");

        assert_eq!(get_matching_answer_count(&groups), 6)
    }
}
