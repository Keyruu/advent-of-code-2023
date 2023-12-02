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
        let mut maybe_number = String::new();

        for char in line.chars() {
            if char.is_numeric() {
                if maybe_number.is_empty() == false {
                    maybe_number = String::new();
                }
                if first == -1 {
                    first = char.to_digit(10).unwrap() as i32;
                }
                if char.is_numeric() {
                    last = char.to_digit(10).unwrap() as i32;
                }
            } else {
                maybe_number.push(char);
                let maybe_number_number = get_kinda_numeric(&maybe_number);

                if maybe_number_number.is_some() {
                    if first == -1 {
                        first = maybe_number_number.unwrap();
                    }
                    last = maybe_number_number.unwrap();
                }
            }
        }
        println!("first {} last {}", first, last);
        line_sum += first * 10 + last;
        println!("line sum {}", line_sum);
    }
    line_sum.to_string()
}

fn get_kinda_numeric(maybe_number: &str) -> Option<i32> {
    let list_of_indexes: Vec<(i32, Option<usize>)> = vec![
        (1, maybe_number.match_indices("one").last().map(|(i, _)| i)),
        (2, maybe_number.match_indices("two").last().map(|(i, _)| i)),
        (3, maybe_number.match_indices("three").last().map(|(i, _)| i)),
        (4, maybe_number.match_indices("four").last().map(|(i, _)| i)),
        (5, maybe_number.match_indices("five").last().map(|(i, _)| i)),
        (6, maybe_number.match_indices("six").last().map(|(i, _)| i)),
        (7, maybe_number.match_indices("seven").last().map(|(i, _)| i)),
        (8, maybe_number.match_indices("eight").last().map(|(i, _)| i)),
        (9, maybe_number.match_indices("nine").last().map(|(i, _)| i))
    ];
    println!("maybe number {}", maybe_number);
    println!("list of indexes {:?}", list_of_indexes);
    // find biggest index
    let mut biggest_index: usize = 0;
    let mut biggest_index_number = -1;
    for (number, index) in list_of_indexes {
        if index.is_some() && index.unwrap() >= biggest_index {
            biggest_index = index.unwrap();
            biggest_index_number = number;
        }
    }
    if biggest_index_number == -1 {
        None
    } else {
        Some(biggest_index_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let answer = part1(input);
        assert_eq!(answer, "281");
    }
}
