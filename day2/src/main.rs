fn get_row(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn get_rows(input: &'static str) -> Vec<Vec<i32>> {
    input.lines().map(get_row).collect()
}

fn safe_row(row: &[i32]) -> bool {
    let inc = row[1] - row[0] > 0;
    row.windows(2).all(|w| {
        let v = w[1] - w[0];
        matches!(if inc { v } else { -v }, 1..=3)
    })
}

fn part_one(input: &'static str) -> usize {
    get_rows(input)
        .into_iter()
        .filter(|row| safe_row(row))
        .count()
}

fn part_two(input: &'static str) -> usize {
    let mut safe = 0;
    let rows = get_rows(input);
    for row in rows {
        for i in 0..row.len() {
            let subset = ([&row[..i], &row[i + 1..]]).concat();
            if safe_row(&subset) {
                safe += 1;
                break;
            }
        }
    }

    safe
}

fn main() {
    let input_1 = include_str!("../../inputs/day2/part1.txt");
    let input_2 = include_str!("../../inputs/day2/part2.txt");

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
        let input = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!(part_one(input), 2);
    }

    #[test]
    fn test_part_two() {
        let input = r"7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
    ";
        assert_eq!(part_two(input), 4);
    }
}
