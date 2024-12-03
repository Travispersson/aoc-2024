fn part_one(input: &str) -> i32 {
    // regex that would match the following:
    // mul(x,y) where x and y are any 3-digit integers and capute the values of x and y
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|cap| {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            a * b
        })
        .sum::<i32>()
}

fn part_two(input: &str) -> i32 {
    // regex that would match the following:
    // mul(x,y) where x and y are any 3-digit integers and capute the values of x and y
    // or don't()
    // or do()
    let re = regex::Regex::new(r"don't\(\)|do\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut enabled = true; // start at enabled

    re.captures_iter(input)
        .filter_map(|cap| {
            enabled = match &cap[0] {
                "don't()" => false,
                "do()" => true,
                _ => enabled,
            };

            if enabled {
                if let (Some(x), Some(y)) = (cap.get(1), cap.get(2)) {
                    let x = x.as_str().parse::<i32>().unwrap();
                    let y = y.as_str().parse::<i32>().unwrap();
                    return Some(x * y);
                }
            }

            None
        })
        .sum::<i32>()
}

fn main() {
    let input_1 = include_str!("../../inputs/day3/part1.txt");
    let input_2 = include_str!("../../inputs/day3/part2.txt");

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
        let input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_one(input), 161);
    }

    #[test]
    fn test_part_two() {
        let input = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_two(input), 48);
    }
}
