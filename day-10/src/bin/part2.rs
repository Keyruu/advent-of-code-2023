use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let input = include_str!("input.txt");
    let result = part2(input);
    println!("Answer: {}", result);
}

fn part1(input: &str) -> String {
    let mut grid = input.lines().map(|line| {
        line.chars().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    let mut start_index = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if char == &'S' {
                start_index = (i, j - 1);
                break;
            }
        }
    }

    let mut current_index = start_index;
    let mut current_origin = 'R';
    let mut result = 1;
    let mut finish = false;
    while !finish {
        let (i, j) = current_index;
        let char = grid[i][j];
        if char == 'S' {
            finish = true;
            break;
        }

        let (next_index, next_origin, exchange_char) = find_next(current_index, char, current_origin);
        grid[i][j] = exchange_char;
        current_index = next_index;
        current_origin = next_origin;

        result += 1;
    }

    let new_grid = grid.iter().map(|row| {
        row.iter().collect::<String>()
    }).collect::<Vec<String>>().join("\n");

    fs::write("foo.txt", new_grid);

    (result / 2).to_string()
}

fn parse_grid(input_str: &str) -> Vec<Vec<char>> {
    // wrap grid with .
    let mut grid: Vec<Vec<char>> = input_str
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| ".".to_owned() + line + ".")
        .map(|line| line.chars().collect())
        .collect();
    grid.insert(0, std::iter::repeat('.').take(grid[0].len()).collect());
    grid.push(std::iter::repeat('.').take(grid[0].len()).collect());
    grid
}

fn find_s(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                return Some((x, y));
            }
        }
    }
    None
}

fn connects(
    grid: &Vec<Vec<char>>,
    (x, y): (usize, usize),
) -> Option<((usize, usize), (usize, usize))> {
    if y >= grid.len() || x >= grid[0].len() {
        return None;
    }
    let item = grid[y][x];
    // results are N, S, E, W
    match item {
        '|' => Some(((x, y.wrapping_sub(1)), (x, y + 1))),
        '-' => Some(((x.wrapping_sub(1), y), (x + 1, y))),
        'L' => Some(((x, y.wrapping_sub(1)), (x + 1, y))),
        'J' => Some(((x.wrapping_sub(1), y), (x, y.wrapping_sub(1)))),
        '7' => Some(((x.wrapping_sub(1), y), (x, y + 1))),
        'F' => Some(((x, y + 1), (x + 1, y))),
        '.' => None,
        'S' => None,
        _ => None,
    }
}

fn get_pipes(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let s = find_s(grid).unwrap();
    let mut curr = s;
    let neighbors = vec![
        (curr.0.wrapping_sub(1), curr.1),
        (curr.0 + 1, curr.1),
        (curr.0, curr.1.wrapping_sub(1)),
        (curr.0, curr.1 + 1),
    ];
    for n in neighbors {
        if n == s {
            continue;
        }
        let connect = connects(grid, n);
        match connect {
            Some((c1, c2)) => {
                if c1 == curr || c2 == curr {
                    curr = n;
                    break;
                }
            }
            None => {}
        };
    }
    let mut pipes: Vec<(usize, usize)> = vec![s];
    while grid[curr.1][curr.0] != 'S' {
        let (c1, c2) = connects(grid, curr).unwrap();
        let next;
        if c1 == *pipes.last().unwrap() {
            next = c2;
        } else {
            next = c1;
        }
        pipes.push(curr);
        curr = next;
    }
    pipes
}

pub fn part2(input: &str) -> String {
    let grid = parse_grid(input);
    let s = find_s(&grid).unwrap();
    let pipes = get_pipes(&grid);
    let pipe_set: HashSet<_> = pipes.clone().into_iter().collect();
    let mut marked_grid = grid;
    let mut prev = (s.0 as i64, s.1 as i64);
    let mut points_to_mark = vec![];
    for i in 0..pipes.len() {
        let segment = pipes[i];
        let curr = (segment.0 as i64, segment.1 as i64);
        // based on direction of piping
        // check points to one side of current and previous pipe segment
        match (curr.0 - prev.0, curr.1 - prev.1) {
            (1, 0) => {
                points_to_mark.push((segment.0, segment.1 + 1));
                points_to_mark.push((segment.0.wrapping_sub(1), segment.1 + 1));
            }
            (0, 1) => {
                points_to_mark.push((segment.0.wrapping_sub(1), segment.1.wrapping_sub(1)));
                points_to_mark.push((segment.0.wrapping_sub(1), segment.1));
            }
            (-1, 0) => {
                points_to_mark.push((segment.0, segment.1.wrapping_sub(1)));
                points_to_mark.push((segment.0 + 1, segment.1.wrapping_sub(1)));
            }
            (0, -1) => {
                points_to_mark.push((segment.0 + 1, segment.1));
                points_to_mark.push((segment.0 + 1, segment.1 + 1));
            }
            _ => {}
        }
        prev = curr;
    }
    for p in points_to_mark {
        marked_grid = search_and_mark(marked_grid, p, &pipe_set);
    }
    let nx = count(&marked_grid, 'X');
    if marked_grid[0][0] == 'X' {
        // if marked squares are outside
        let total = marked_grid.len() * marked_grid[0].len();
        (total - nx - pipe_set.len()).to_string()
    } else {
        // if marked squares are inside
        nx.to_string()
    }
}

fn search_and_mark(
    grid: Vec<Vec<char>>,
    curr: (usize, usize),
    pipes: &HashSet<(usize, usize)>,
) -> Vec<Vec<char>> {
    if curr.1 >= grid.len() || curr.0 >= grid[0].len() {
        return grid;
    }
    if grid[curr.1][curr.0] == 'X' {
        return grid;
    }
    if pipes.contains(&curr) {
        return grid;
    }
    let neighbors = vec![
        (curr.0.wrapping_sub(1), curr.1),
        (curr.0 + 1, curr.1),
        (curr.0, curr.1.wrapping_sub(1)),
        (curr.0, curr.1 + 1),
    ];
    let mut g = grid;
    g[curr.1][curr.0] = 'X';
    for n in neighbors {
        g = search_and_mark(g, n, pipes);
    }
    g
}

fn count(grid: &Vec<Vec<char>>, c: char) -> usize {
    let mut n = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == c {
                n += 1;
            }
        }
    }
    n
}

fn find_next(index: (usize, usize), pipe_type: char, origin: char) -> ((usize, usize), char, char) {
    let mut next_index = (0, 0);
    let mut next_origin = ' ';
    let mut exchange_char = ' ';
    let (i, j) = index;
    match pipe_type {
        'F' => {
            exchange_char = '┌';
            match origin {
                'D' => {
                    next_index = (i, j + 1);
                    next_origin = 'L';
                },
                'R' => {
                    next_index = (i + 1, j);
                    next_origin = 'U';
                },
                _ => {panic!("wtf")}
            }
        },
        'J' => {
            exchange_char = '┘';
            match origin {
                'U' => {
                    next_index = (i, j - 1);
                    next_origin = 'R';
                },
                'L' => {
                    next_index = (i - 1, j);
                    next_origin = 'D';
                },
                _ => {panic!("wtf")}
            }
        },
        'L' => {
            exchange_char = '└';
            match origin {
                'U' => {
                    next_index = (i, j + 1);
                    next_origin = 'L';
                },
                'R' => {
                    next_index = (i - 1, j);
                    next_origin = 'D';
                },
                _ => {panic!("wtf")}
            }
        },
        '7' => {
            exchange_char = '┐';
            match origin {
                'D' => {
                    next_index = (i, j - 1);
                    next_origin = 'R';
                },
                'L' => {
                    next_index = (i + 1, j);
                    next_origin = 'U';
                },
                _ => {panic!("wtf")}
            }
        },
        '|' => {
            exchange_char = '│';
            match origin {
                'U' => {
                    next_index = (i + 1, j);
                    next_origin = 'U';
                },
                'D' => {
                    next_index = (i - 1, j);
                    next_origin = 'D';
                },
                _ => {panic!("wtf")}
            }
        },
        '-' => {
            exchange_char = '─';
            match origin {
                'L' => {
                    next_index = (i, j + 1);
                    next_origin = 'L';
                },
                'R' => {
                    next_index = (i, j - 1);
                    next_origin = 'R';
                },
                _ => {panic!("wtf")}
            }
        },
        _ => {panic!("wtf")}
    }

    (next_index, next_origin, exchange_char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        let result = part1(input);

        assert_eq!(result, "8");
    }
}
