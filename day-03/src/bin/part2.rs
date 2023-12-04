fn main() {
    let input = include_str!("input1.txt");
    let answer = part1(input);
    println!("answer {}", answer);
    println!("{}", '.'.is_numeric())
}

fn part1(input: &str) -> String {
    let mut sum_of_parts = 0;
    let special_chars = "$!@#^%&*()/?-+,<>{}[]|~=_";
    let lines = input.lines().collect::<Vec<&str>>();
    let two_d = lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    two_d.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, char) | {
            if special_chars.contains(*char) {
                println!("symbol");
                let above_line = two_d.get(i - 1).unwrap();
                let below_line = two_d.get(i + 1).unwrap();

                let mut parts = get_sum_of_parts_in_line(above_line, j);
                parts.append(&mut get_sum_of_parts_in_line(line, j));
                parts.append(&mut get_sum_of_parts_in_line(below_line, j));

                if parts.len() == 2 {
                    sum_of_parts += parts[0] * parts[1];
                }

                println!("{} {} {}", above_line.get(j - 1).unwrap(), above_line.get(j).unwrap(), above_line.get(j + 1).unwrap());
                println!("{} {} {}", line.get(j - 1).unwrap(), char, line.get(j + 1).unwrap());
                println!("{} {} {}", below_line.get(j - 1).unwrap(), below_line.get(j).unwrap(), below_line.get(j + 1).unwrap());

            }
        });
        println!();
    });
    sum_of_parts.to_string()
}

fn get_sum_of_parts_in_line(line: &Vec<char>, index: usize) -> Vec<u32> {
    let mut parts: Vec<u32> = vec![];
    let left = line.get(index - 1).unwrap();
    let middle = line.get(index).unwrap();
    let right = line.get(index + 1).unwrap();

    let allowed_chars = '0'..='9';
    let is_left_number = allowed_chars.contains(left);
    let is_middle_number = allowed_chars.contains(middle);
    let is_right_number = allowed_chars.contains(right);

    let all_number = is_left_number && is_middle_number && is_right_number;
    let left_and_right_number = is_left_number && is_right_number && !is_middle_number;
    let left_and_middle_number = is_left_number && is_middle_number && !is_right_number;
    let middle_and_right_number = is_middle_number && is_right_number && !is_left_number;
    let only_left_number = is_left_number && !is_middle_number && !is_right_number;
    let only_right_number = is_right_number && !is_middle_number && !is_left_number;
    let only_middle_number = is_middle_number && !is_left_number && !is_right_number;

    if all_number {
        parts.push(left.to_digit(10).unwrap() * 100
            + middle.to_digit(10).unwrap() * 10
            + right.to_digit(10).unwrap())
    }
    if left_and_right_number || only_left_number {
        if line.get(index - 2).unwrap().is_numeric() == false {
            parts.push(left.to_digit(10).unwrap())
        } else if line.get(index - 3).unwrap().is_numeric() == false {
            parts.push(line.get(index - 2).unwrap().to_digit(10).unwrap() * 10
                + left.to_digit(10).unwrap())
        } else {
            parts.push(line.get(index - 3).unwrap_or(&'0').to_digit(10).unwrap_or(0) * 100
                + line.get(index - 2).unwrap_or(&'0').to_digit(10).unwrap_or(0) * 10
                + left.to_digit(10).unwrap_or(0))
        }
    }

    if left_and_middle_number {
        parts.push(line.get(index - 2).unwrap_or(&'0').to_digit(10).unwrap_or(0) * 100
            + left.to_digit(10).unwrap_or(0) * 10
            + middle.to_digit(10).unwrap_or(0))
    }

    if middle_and_right_number {
        if line.get(index + 2).unwrap().is_numeric() {
            parts.push(middle.to_digit(10).unwrap_or(0) * 100
                + right.to_digit(10).unwrap_or(0) * 10
                + line.get(index + 2).unwrap_or(&'0').to_digit(10).unwrap_or(0))
        } else {
            parts.push(middle.to_digit(10).unwrap_or(0) * 10
                + right.to_digit(10).unwrap_or(0))
        }
    }

    if left_and_right_number || only_right_number {
        if line.get(index + 2).unwrap().is_numeric() == false  {
            parts.push(right.to_digit(10).unwrap())
        } else if line.get(index + 3).unwrap().is_numeric() == false {
            parts.push(right.to_digit(10).unwrap() * 10
                + line.get(index + 2).unwrap().to_digit(10).unwrap())
        } else {
            parts.push(right.to_digit(10).unwrap() * 100
                + line.get(index + 2).unwrap().to_digit(10).unwrap() * 10
                + line.get(index + 3).unwrap().to_digit(10).unwrap())
        }
    }

    if only_middle_number {
        parts.push(middle.to_digit(10).unwrap())
    }

    println!("parts {:?}", parts);

    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let answer = part1(input);
        assert_eq!(answer, "467835");
    }
}