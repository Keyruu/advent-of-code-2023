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

    let mut current_map: HashMap<&&str, u128> = map.keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| (k, 0))
        .collect();


    let lcm_numbers = current_map.par_iter().map(|(k, v)| {
        let mut current = *k;
        let mut found = false;
        let mut result: u128 = 0;
        while !found {
            for char in instructions.chars() {
                let mut index = 0;
                match char {
                    'L' => {
                        index = 0;
                    },
                    'R' => {
                        index = 1;
                    },
                    _ => {}
                }

                current = &map.get(current).unwrap()[index];

                result += 1;

                if current.ends_with('Z') {
                    println!("found");
                    found = true;
                    break;
                }
            }
        }

        result
    }).collect::<Vec<u128>>();


    lcm(lcm_numbers).to_string()
}

pub fn lcm(nums: Vec<u128>) -> u128 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(nums[1..].to_vec());
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = part1(input);
        assert_eq!(result, "6");
    }
}