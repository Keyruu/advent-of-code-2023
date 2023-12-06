use std::alloc::take_alloc_error_hook;

fn main() {
    let input = include_str!("input1.txt");
    let answer = part1(input);
    println!("Answer: {}", answer);
}

fn part1(input: &str) -> String {
    let seeds_and_maps = input.split("\n\n").collect::<Vec<&str>>();
    let seeds = seeds_and_maps[0].split(": ").collect::<Vec<&str>>()[1];
    let seed_strings = seeds.split(" ").collect::<Vec<&str>>();
    let seed_numbers_and_ranges = seed_strings
        .iter()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut seed_numbers: Vec<i64> = vec![];

    // for (i, seed_or_range) in seed_numbers_and_ranges.iter().enumerate() {
    //     if i % 2 == 0 {
    //         let seed = seed_or_range.clone();
    //         let range = seed_numbers_and_ranges[i + 1];
    //         let end_of_range = seed + range - 1;
    //         for j in seed..=end_of_range {
    //             seed_numbers.push(j);
    //         }
    //     }
    // }

    let maps_as_string = seeds_and_maps.iter().skip(1).collect::<Vec<&&str>>();
    let maps = maps_as_string
        .iter()
        .map(|m| {
            m.split("\n")
                .skip(1)
                .map(|r| {
                    r.split(" ")
                        .filter(|n| !n.is_empty())
                        .map(|n| n.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>()
                })
                .collect::<Vec<Vec<i64>>>()
        })
        .collect::<Vec<Vec<Vec<i64>>>>();

    let mut targets: Vec<i64> = vec![];
    for seed in seed_numbers {
        let mut target = seed;
        for map in maps.clone() {
            println!("map {:?}", map);
            target = get_target_for_source(target, &map);
            println!("seed {} target {}", seed, target);
        }
        targets.push(target);
    }

    let smallest = targets.iter().min().unwrap();

    smallest.to_string()
}

fn get_target_for_source(source: i64, map: &Vec<Vec<i64>>) -> i64 {
    let mut target = -1;

    for row in map {
        if source >= row[1] && source <= row[1] + row[2] - 1 {
            target = row[0] + (source - row[1]);
        }
    }

    if target == -1 {
        target = source;
    }

    target
}

fn get_source_for_target(target: i64, map: &Vec<Vec<i64>>) -> Vec<i64> {
    let mut sources: Vec<i64> = vec![];

    for row in map {
        if target >= row[0] && target <= row[0] + row[2] - 1 {
            sources.push(row[1] + (target - row[0]));
        }
    }

    sources.push(target);

    sources
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part1(input), "46");
    }
}
