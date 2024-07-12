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

    let seed_ranges: Vec<Vec<i64>> = seed_numbers_and_ranges
        .chunks(2)
        .map(|chunk| vec![chunk[0], chunk[0] + chunk[1] - 1])
        .collect();

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

    let mut smallest = -1i64;

    for range in seed_ranges.clone() {
        println!("range {:?}", range);
        let mut mapped_ranges = vec![range.clone()];
        for (i, map) in maps.clone().into_iter().enumerate() {
            mapped_ranges = get_seed_ranges_for_map(mapped_ranges, &map);
            println!("mapped_ranges {:?}", mapped_ranges);
            if i == maps.len() - 1 {
                mapped_ranges.sort_by(|a, b| a[0].cmp(&b[0]));
                if smallest == -1 || mapped_ranges[0][0] < smallest {
                    smallest = mapped_ranges[0][0];
                }
            }
        }
    }

    smallest.to_string()
}

fn get_seed_ranges_for_map(range: Vec<i64>, map: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut mapped_ranges: Vec<Vec<i64>> = vec![];
    let mut unknown_ranges: Vec<Vec<i64>> = vec![];

    let mapped_range_len = mapped_ranges.len();
    let range_start = range[0];
    let range_end = range[1];

    for row in map {
        let row_range_start = row[1];
        let row_range_end = row[1] + row[2] - 1;
        let offset = row[0] - row[1];

        let is_in_range = row_range_start <= range_start && row_range_end >= range_end;
        let is_not_in_range = row_range_start > range_end || row_range_end < range_start;

        if range_start + offset == 0 {
            println!("row {:?}", row);
        }

        if !is_in_range && !is_not_in_range {
            if range_start > row_range_start {
                let unknown_range_already_there_index =
                    unknown_ranges.iter().position(|r| r[0] == range_start);

                if unknown_range_already_there_index.is_some() {
                    unknown_ranges.remove(unknown_range_already_there_index.unwrap());
                }
                unknown_ranges.push(vec![row_range_end + 1, range_end]);

                mapped_ranges.push(vec![range_start + offset, row_range_end + offset]);
            } else {
                let unknown_range_already_there_index =
                    unknown_ranges.iter().position(|r| r[1] == range_end);

                if unknown_range_already_there_index.is_some() {
                    unknown_ranges.remove(unknown_range_already_there_index.unwrap());
                }
                unknown_ranges.push(vec![range_start, row_range_start - 1]);

                mapped_ranges.push(vec![row_range_start + offset, range_end + offset]);
            }
        } else if is_in_range {
            mapped_ranges.push(vec![range_start + offset, range_end + offset]);
        }
    }
    if mapped_ranges.len() == mapped_range_len {
        mapped_ranges.push(range);
    }


    if unknown_ranges.len() > 0 {
        mapped_ranges.extend(unknown_ranges);
    }

    mapped_ranges
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
