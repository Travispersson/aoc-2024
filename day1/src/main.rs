fn lists(input: &'static str) -> (Vec<i64>, Vec<i64>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let a = parts.next().unwrap().parse::<i64>().unwrap();
        let b = parts.next().unwrap().parse::<i64>().unwrap();
        left.push(a);
        right.push(b);
    }

    (left, right)
}

fn part_one(input: &'static str) -> i64 {
    let (mut left, mut right) = lists(input);
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i64>()
}

fn part_two(input: &'static str) -> i64 {
    let (left, right) = lists(input);
    let mut counter = std::collections::HashMap::new();
    right.iter().for_each(|x| {
        *counter.entry(*x).or_insert(0) += 1;
    });

    let mut sum = 0;
    for n in left.iter() {
        sum += n * counter.get(n).unwrap_or(&0);
    }

    sum
}

fn main() {
    let input_1 = include_str!("../../inputs/day1/part1.txt");
    let input_2 = include_str!("../../inputs/day1/part2.txt");

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
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        assert_eq!(part_one(input), 11);
    }

    #[test]
    fn test_part_two() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        assert_eq!(part_two(input), 31);
    }
}
