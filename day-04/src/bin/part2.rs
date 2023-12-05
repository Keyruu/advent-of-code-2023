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
    let mut game_number = 0;
    lines.enumerate().for_each(|(i, line)| {
        let line_split = line.split(": ").collect::<Vec<&str>>();
        let maybe_game_number = line_split[0].split(" ").collect::<Vec<&str>>();
        game_number = maybe_game_number.last().unwrap().parse::<i32>().unwrap();
        let stripped_line = line_split[1];
        let ranges = stripped_line.split(" | ").collect::<Vec<&str>>();
        let winning_numbers = ranges[0].split(" ")
            .filter(|n| !n.is_empty()).collect::<Vec<&str>>();
        let scratch_numbers = ranges[1].split(" ")
            .filter(|n| !n.is_empty()).collect::<Vec<&str>>();
        let actual_wins = scratch_numbers.iter()
            .filter(|&n| winning_numbers.contains(n)).collect::<Vec<&&str>>();

        let curennt_copies = game_map.get(&game_number).unwrap_or(&0).clone();

        for (j, _) in actual_wins.iter().enumerate() {
            let copy_number = (game_number as i32) + (j as i32) + 1;
            let copies = &game_map.get(&copy_number).unwrap_or(&0).clone();
            game_map.insert(copy_number, copies + (curennt_copies + 1));
        };

        println!("sum {}", sum)
    });
    sum += game_map.values().sum::<i32>() + game_number;
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
        assert_eq!(part1(input), "30");
    }
}