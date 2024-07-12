use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);
    println!("Answer: {}", result);
}

fn part1(input: &str) -> String {
    let mut sequences = input
        .lines()
        .map(|line| line.split(" ")
            .map(|num| i32::from_str(num).unwrap())
            .collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let rev_sequences = sequences.iter()
        .map(|sequence| sequence.iter().rev()
            .map(|n| *n).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let result: i32 = rev_sequences.iter()
        .map(|sequence| find_sequence(sequence, true)).sum();

    result.to_string()
}

fn find_sequence(sequence: &Vec<i32>, is_top: bool) -> i32 {
    let mut new_sequence = Vec::new();
    for (i, num) in sequence.iter().enumerate() {
       if i != sequence.len() - 1 {
           new_sequence.push(sequence[i + 1] - num);
       }
    }
    return if new_sequence.iter().all(|n| n == &0) {
        0
    } else {
        let next_in_sequence = find_sequence(&new_sequence, false);
        let last_num = sequence[sequence.len() - 1];
        let last_in_new_sequence = new_sequence[new_sequence.len() - 1];
        return if is_top {
            last_num + last_in_new_sequence + next_in_sequence
        } else {
            last_in_new_sequence + next_in_sequence
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = part1(input);

        assert_eq!(result, "2");
    }
}