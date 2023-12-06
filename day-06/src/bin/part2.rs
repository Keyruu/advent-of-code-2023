fn main() {
    let input = include_str!("input.txt");
    let answer = part1(input);
    println!("Answer {}", answer);
}

fn part1(input: &str) -> String {
    let time_and_distance: Vec<&str> = input.split("\n").collect();
    let time: i64 = time_and_distance[0]
        .split(" ")
        .skip(1)
        .filter(|t| !t.is_empty())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    let distance: i64 = time_and_distance[1]
        .split(" ")
        .skip(1)
        .filter(|d| !d.is_empty())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    let mut amounts_of_better_times: Vec<i64> = Vec::new();
    let amount_of_better_times = get_amount_of_better_times(time, distance);
    amounts_of_better_times.push(amount_of_better_times);
    println!("Better times {}", amount_of_better_times);

    let result = amounts_of_better_times.into_iter().reduce(|a, b| a * b);
    result.unwrap().to_string()
}

fn get_amount_of_better_times(time: i64, distance: i64) -> i64 {
    let mut better_times = Vec::new();
    for i in 0..=time {
        let mut maybe = 0;
        if i != 0 && i != time {
            maybe = (time - i) * i;
        }
        if maybe > distance {
            better_times.push(i);
        }
    }
    better_times.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part1(input), "71503");
    }
}
