fn main() {
    let input = include_str!("input1.txt");
    let answer = part1(input);
    println!("{}", answer);
}

fn part1(input: &str) -> String {
    let mut line_sum = 0;
    for line in input.lines() {
        let mut first = -1;
        let mut last = -1;
        for char in line.chars() {
            if first == -1 && char.is_numeric() {
                first = char.to_digit(10).unwrap() as i32;
            }
            if char.is_numeric() {
                last = char.to_digit(10).unwrap() as i32;
            }
        }
        line_sum += first * 10 + last;
        println!("{}", line_sum);
    }
    line_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let answer = part1(input);
        assert_eq!(answer, "142");
    }
}