fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);
    println!("Answer: {}", result);
}

fn part1(input: &str) -> String {
    let lines = input.lines().clone();
    let galaxies: Vec<(i32, i32)> = lines
        .clone()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect();

    let lines_as_vec: Vec<&str> = lines.collect();
    let empty_rows: Vec<i32> = lines_as_vec
        .iter()
        .enumerate()
        .map(|(y, line)| y as i32)
        .filter(|y| lines_as_vec[*y as usize].chars().all(|c| c == '.'))
        .collect();

    let mut empty_cols: Vec<i32> = Vec::new();
    for x in 0..lines_as_vec[0].len() {
        let mut empty = true;
        for y in 0..lines_as_vec.len() {
            if lines_as_vec[y].chars().nth(x).unwrap() == '#' {
                empty = false;
                break;
            }
        }

        if empty {
            empty_cols.push(x as i32);
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

            let expanded_cols = empty_cols
                .iter()
                .filter(|x| **x > x1 && **x < x2 || **x > x2 && **x < x1)
                .count();
            let expanded_rows = empty_rows
                .iter()
                .filter(|y| **y > y1 && **y < y2 || **y > y2 && **y < y1)
                .count();

            result += dx.abs() + dy.abs() + expanded_cols as i32 + expanded_rows as i32;
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
        assert_eq!(result, "374");
    }
}
