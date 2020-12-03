use std::fs;
use std::collections::HashSet;

fn get_expenses(filename: &str) -> HashSet<u32> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let expenses: HashSet<u32> = contents
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    expenses
}

fn find_pair(expenses: &HashSet<u32>) -> u32 {
    for expense in expenses {
        let diff = 2020 - expense;
        if expenses.contains(&diff) {
            return expense * diff;
        }
    }

    0
}

fn find_trio(expenses: &HashSet<u32>) -> u32 {
    for expense_1 in expenses {
        for expense_2 in expenses {
            if expense_1 + expense_2 > 2020 {
                continue;
            }

            let diff = 2020 - expense_1 - expense_2;
            if expenses.contains(&diff) {
                return expense_1 * expense_2 * diff;
            }
        }
    }

    0
}

fn main() {
    let expenses = get_expenses("input.txt");

    println!("Part 1 Result: {}", find_pair(&expenses));
    println!("Part 2 Result: {}", find_trio(&expenses));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pair() {
        let expenses = get_expenses("test.txt");

        assert_eq!(find_pair(&expenses), 514579)
    }

    #[test]
    fn test_trio() {
        let expenses = get_expenses("test.txt");

        assert_eq!(find_trio(&expenses), 241861950)
    }
}