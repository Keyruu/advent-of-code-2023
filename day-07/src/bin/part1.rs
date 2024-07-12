use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use crate::Hand::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, Pair, ThreeOfAKind, TwoPair};

fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);
    println!("Answer: {}", result);
}

fn part1(input: &str) -> String {
    let mut hands_and_bids = input.lines()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    hands_and_bids.sort_by(|a, b| compare(a[0], b[0]));

    let result: i32 = hands_and_bids.iter().enumerate().map(|(i, hand_and_bid)| {
        hand_and_bid[1].parse::<i32>().unwrap() * (i as i32 + 1)
    }).sum();

    result.to_string()
}

fn compare(a: &str, b: &str) -> Ordering {
    let a_type = get_hand_type(a);
    let b_type = get_hand_type(b);

    return if a_type == b_type {
        compare_same_type(a, b)
    } else {
        return if a_type > b_type {
            Greater
        } else {
            Less
        };
    }
}

fn compare_same_type(a: &str, b: &str) -> Ordering {
    let a_chars = a.chars().collect::<Vec<char>>();
    let b_chars = b.chars().collect::<Vec<char>>();

    for i in 0..a_chars.len() {
        if a_chars[i] != b_chars[i] {
            return if get_card_value(a_chars[i]) > get_card_value(b_chars[i]) {
                Greater
            } else {
                Less
            };
        }
    }

    return Equal;
}

fn get_card_value(card: char) -> u8 {
    match card {
        'A' => {
            14
        },
        'K' => {
            13
        },
        'Q' => {
            12
        },
        'J' => {
            11
        },
        'T' => {
            10
        },
        _ => {
            card.to_digit(10).unwrap() as u8
        }
    }

}

fn get_hand_type(hand: &str) -> Hand {
    let mut map = HashMap::new();

    for char in hand.chars() {
        let count = map.entry(char).or_insert(0);
        *count += 1;
    }

    let hand_type = match map.len() {
        1 => {
            FiveOfAKind
        },
        2 => {
            if map.values().any(|&v| v == 4) {
                FourOfAKind
            } else {
                FullHouse
            }
        },
        3 => {
            if map.values().any(|&v| v == 3) {
                ThreeOfAKind
            } else {
                TwoPair
            }
        },
        4 => {
            Pair
        },
        5 => {
            HighCard
        },
        _ => {
            panic!("Invalid hand");
        }
    };

    hand_type
}

#[derive(PartialOrd, PartialEq, Eq, Debug)]
enum Hand {
    HighCard = 0,
    Pair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = part1(input);
        assert_eq!(result, "6440");
    }
}