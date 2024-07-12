fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);
    println!("Answer: {}", result);
}

fn part1(input: &str) -> String {
    let lines = input.lines().clone();
    let galaxies: Vec<(i128, i128)> = lines
        .clone()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i128, y as i128))
        })
        .collect();

    let lines_as_vec: Vec<&str> = lines.collect();
    let empty_rows: Vec<i128> = lines_as_vec
        .iter()
        .enumerate()
        .map(|(y, line)| y as i128)
        .filter(|y| lines_as_vec[*y as usize].chars().all(|c| c == '.'))
        .collect();

    let mut empty_cols: Vec<i128> = Vec::new();
    for x in 0..lines_as_vec[0].len() {
        let mut empty = true;
        for y in 0..lines_as_vec.len() {
            if lines_as_vec[y].chars().nth(x).unwrap() == '#' {
                empty = false;
                break;
            }
        }

        if empty {
            empty_cols.push(x as i128);
        }
    }

    println!("Empty rows: {:?}", empty_rows);
    println!("Empty cols: {:?}", empty_cols);

    let n = galaxies.len();
    let mut permutations = 0;
    let mut result = 0;
    for i in 0..n {
        for j in 0..n {
            if j <= i {
                continue;
            }

            println!("{} {}", i, j);
            let (x1, y1) = galaxies[i];
            let (x2, y2) = galaxies[j];

            let dx = x1 - x2;
            let dy = y1 - y2;

            let mut expanded_cols = empty_cols
                .iter()
                .filter(|x| **x > x1 && **x < x2 || **x > x2 && **x < x1)
                .count();
            let mut expanded_rows = empty_rows
                .iter()
                .filter(|y| **y > y1 && **y < y2 || **y > y2 && **y < y1)
                .count();

            expanded_cols = expanded_cols * 1000000 - expanded_cols;
            expanded_rows = expanded_rows * 1000000 - expanded_rows;

            result += dx.abs() + dy.abs() + expanded_cols as i128 + expanded_rows as i128;
        }
    }

    println!("Permutations: {}", permutations);

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let result = part1(input);
        assert_eq!(result, "8410");
    }
}
