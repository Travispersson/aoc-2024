use std::{
    cmp::Ordering,
    collections::{hash_set, HashMap, HashSet},
};

type Rules = HashMap<i32, HashSet<i32>>;

fn get_rules(input: &str) -> Rules {
    let mut rules = HashMap::new();
    input.lines().for_each(|line| {
        let rule = line.split("|").collect::<Vec<&str>>();
        let a = rule[0].parse::<i32>().unwrap();
        let b = rule[1].parse::<i32>().unwrap();
        rules.entry(a).or_insert(hash_set::HashSet::new()).insert(b);
    });
    rules
}

fn get_page_numbers(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split(",").map(|d| d.parse::<i32>().unwrap()).collect())
        .collect()
}

fn part_one(input: &str) -> i32 {
    let split = input.split("\n\n").collect::<Vec<&str>>();
    let rules = get_rules(split[0]);
    let page_nums = get_page_numbers(split[1]);

    let mut total = 0;
    for row in page_nums {
        if row.is_sorted_by(|a, b| {
            if let Some(set) = rules.get(a) {
                if set.contains(b) {
                    return true;
                }
            }
            if let Some(set) = rules.get(b) {
                if set.contains(a) {
                    return false;
                }
            }

            true
        }) {
            // save middle number in row
            total += row[row.len() / 2];
        }
    }

    total
}

fn part_two(input: &str) -> i32 {
    let split = input.split("\n\n").collect::<Vec<&str>>();
    let rules = get_rules(split[0]);
    let page_nums = get_page_numbers(split[1]);

    let cmp = |a, b| {
        if let Some(set) = rules.get(a) {
            if set.contains(b) {
                return true;
            }
        }
        if let Some(set) = rules.get(b) {
            if set.contains(a) {
                return false;
            }
        }

        true
    };

    let mut total = 0;
    for row in page_nums.iter() {
        if !row.is_sorted_by(cmp) {
            let mut r = row.clone();
            r.sort_by(|a, b| {
                if let Some(set) = rules.get(a) {
                    if set.contains(b) {
                        return Ordering::Less;
                    }
                }
                if let Some(set) = rules.get(b) {
                    if set.contains(a) {
                        return Ordering::Greater;
                    }
                }

                Ordering::Equal
            });

            total += r[r.len() / 2];
        }
    }

    total
}

fn main() {
    let input_1 = include_str!("../../inputs/day5/part1.txt");
    let input_2 = include_str!("../../inputs/day5/part2.txt");

    let p1 = part_one(input_1);
    println!("Result part 1: {}", p1);
    let p2 = part_two(input_2);
    println!("Result part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(part_one(input), 143);
    }

    #[test]
    fn test_part_two() {
        let input = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(part_two(input), 123);
    }
}
