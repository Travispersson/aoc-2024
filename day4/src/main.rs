type Matrix = Vec<Vec<char>>;

fn get_matrix(input: &str) -> Matrix {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

const NEIGHBOURS_ALL_DIRECTIONS: [(i32, i32); 8] = [
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

fn part_one(input: &str) -> usize {
    let matrix = get_matrix(input);
    let mut total = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let p = (i, j);
            if matrix[i][j] == 'X' {
                for (dx, dy) in NEIGHBOURS_ALL_DIRECTIONS.iter() {
                    let mut pattern = Vec::with_capacity(4);
                    pattern.push('X');
                    for n in 1..4 {
                        let _x = p.0 as i32 + dx * n;
                        let _y = p.1 as i32 + dy * n;
                        if _x < 0 || _y < 0 {
                            // out of bounds
                            break;
                        }

                        if (_x as usize >= matrix.len())
                            || (_y as usize >= matrix[_x as usize].len())
                        {
                            // out of bounds
                            break;
                        }

                        pattern.push(matrix[_x as usize][_y as usize]);
                    }

                    if "XMAS" == pattern.iter().collect::<String>() {
                        total += 1
                    }
                }
            }
        }
    }

    total
}

const NEIGHBOURS_DIAGONAL: [(i32, i32); 4] = [(-1, 1), (1, 1), (1, -1), (-1, -1)];

fn part_two(input: &str) -> usize {
    let matrix = get_matrix(input);
    // M.M
    // .A.
    // S.S

    // S.S
    // .A.
    // M.M

    // M.S
    // .A.
    // M.S

    // S.M
    // .A.
    // S.M
    let patterns = ["AMMSS", "ASSMM", "AMSSM", "ASMMS"];
    let mut total = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let p = (i, j);
            if matrix[i][j] == 'A' {
                let mut pattern = Vec::with_capacity(5);
                pattern.push('A');
                for (dx, dy) in NEIGHBOURS_DIAGONAL.iter() {
                    let _x = p.0 as i32 + dx;
                    let _y = p.1 as i32 + dy;
                    if _x < 0 || _y < 0 {
                        // out of bounds
                        break;
                    }

                    if (_x as usize >= matrix.len()) || (_y as usize >= matrix[_x as usize].len()) {
                        // out of bounds
                        break;
                    }

                    pattern.push(matrix[_x as usize][_y as usize]);
                }
                let pattern = pattern.iter().collect::<String>();
                if patterns.contains(&pattern.as_str()) {
                    total += 1;
                }
            }
        }
    }

    total
}

fn main() {
    let input_1 = include_str!("../../inputs/day4/part1.txt");
    let input_2 = include_str!("../../inputs/day4/part2.txt");

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
        let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part_one(input), 18);
    }

    #[test]
    fn test_part_two_minimal() {
        let input = r"MPS
XAS
MPS";
        assert_eq!(part_two(input), 1);
    }

    #[test]
    fn test_part_two() {
        let input = r".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!(part_two(input), 9);
    }
}
