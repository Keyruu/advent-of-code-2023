use std::collections::HashMap;

fn main() {
    let input = include_str!("input1.txt");
    let answer = part1(input);
    println!("Answer: {}", answer);
}

fn part1(input: &str) -> String {
    let lines = input.lines();
    let mut sum = 0;
    let mut game_map: HashMap<i32, i32> = HashMap::new();
    lines.enumerate().for_each(|(i, line)| {
        let game_number = i + 1;
        let stripped_line = line.split(": ").collect::<Vec<&str>>()[1];
        let ranges = stripped_line.split(" | ").collect::<Vec<&str>>();
        let winning_numbers = ranges[0].split(" ")
            .filter(|n| !n.is_empty()).collect::<Vec<&str>>();
        let scratch_numbers = ranges[1].split(" ")
            .filter(|n| !n.is_empty()).collect::<Vec<&str>>();
        let actual_wins = scratch_numbers.iter()
            .filter(|&n| winning_numbers.contains(n)).collect::<Vec<&&str>>();
        println!("winning_numbers {:?}", winning_numbers);
        println!("scratch_numbers {:?}", scratch_numbers);
        println!("{:?}", actual_wins);

        if actual_wins.len() == 1 {
            sum += 1;
        } else if actual_wins.len() > 1 {
            sum += 2i32.pow(actual_wins.len() as u32 - 1);
        }
    });
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(input), "13");
    }
}