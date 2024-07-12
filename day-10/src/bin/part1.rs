fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);
    println!("Answer: {}", result);
}

fn part1(input: &str) -> String {
    let grid = input.lines().map(|line| {
        line.chars().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    let mut start_index = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if char == &'S' {
                start_index = (i, j + 1);
                break;
            }
        }
    }

    let mut current_index = start_index;
    let mut current_origin = 'L';
    let mut result = 1;
    let mut finish = false;
    while !finish {
        let (i, j) = current_index;
        let char = grid[i as usize][j as usize];
        if char == 'S' {
            finish = true;
            break;
        }

        let (next_index, next_origin) = find_next(current_index, char, current_origin);
        current_index = next_index;
        current_origin = next_origin;

        result += 1;
    }

    (result / 2).to_string()
}

fn find_next(index: (usize, usize), pipe_type: char, origin: char) -> ((usize, usize), char) {
    let mut next_index = (0, 0);
    let mut next_origin = ' ';
    let (i, j) = index;
    match pipe_type {
        'F' => {
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
        'S' => {
            next_index = (0,0);
            next_origin = 'S';
        },
        _ => {panic!("wtf")}
    }

    (next_index, next_origin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

        let result = part1(input);

        assert_eq!(result, "8");
    }
}
