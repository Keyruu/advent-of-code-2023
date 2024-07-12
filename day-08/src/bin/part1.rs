use std::collections::HashMap;
use rayon::prelude::*;

fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);
    println!("Answer: {}", result);
}

fn part1(input: &str) -> String {
    let instruction_and_map = input.split("\n\n").collect::<Vec<&str>>();
    let instructions = instruction_and_map[0];
    let map_as_string = instruction_and_map[1];

    let map: HashMap<&str, Vec<&str>> = map_as_string.lines().par_bridge().map(|line| {
        let mut parts = line.split(" = ");
        let key = parts.next().unwrap();
        let mut value = parts.next().unwrap();
        value = value.trim_start_matches('(');
        value = value.trim_end_matches(')');
        let l_and_r = value.split(", ").collect::<Vec<&str>>();

        (key, l_and_r)
    }).collect();

    let mut found = false;
    let mut result = 0;
    let mut current = "AAA";
    while !found {
        for char in instructions.chars() {
            match char {
                'L' => {
                    current = map.get(current).unwrap()[0];
                },
                'R' => {
                    current = map.get(current).unwrap()[1];
                },
                _ => {}
            }

            result += 1;

            if current == "ZZZ" {
                found = true;
                break;
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = part1(input);
        assert_eq!(result, "6");
    }
}