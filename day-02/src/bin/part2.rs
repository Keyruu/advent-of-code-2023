fn main() {
    let input = include_str!("input1.txt");
    let answer = part1(input);
    println!("answer {}", answer);
}

fn part1(input: &str) -> String {
    let mut power_of_sum = 0;
    for line in input.lines() {
        let mut highest_red = 0;
        let mut highest_green = 0;
        let mut highest_blue = 0;

        let colon_split = line.split(":").collect::<Vec<&str>>();
        let game = colon_split[0];
        let cubes_string = colon_split[1];
        let game_split = game.split(" ").collect::<Vec<&str>>();
        let game_id = game_split[1];
        let subsets = cubes_string.split(";").collect::<Vec<&str>>();
        for subset in subsets {
            let cubes = subset.split(",").collect::<Vec<&str>>();
            for cube in cubes {
                let trimmed_cube = cube.trim_start();
                let trimmed_cube_split = trimmed_cube.split(" ").collect::<Vec<&str>>();
                let cube_amount = trimmed_cube_split[0].parse::<i32>().unwrap();
                let cube_color = trimmed_cube_split[1];

                match cube_color {
                    "red" => if cube_amount > highest_red {
                        highest_red = cube_amount;
                    },
                    "green" => if cube_amount > highest_green {
                        highest_green = cube_amount;
                    },
                    "blue" => if cube_amount > highest_blue {
                        highest_blue = cube_amount;
                    },
                    _ => continue
                }
            }
        }
            power_of_sum += highest_blue * highest_green * highest_red;
    }
    power_of_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let answer = part1(input);
        assert_eq!(answer, "2286");
    }
}
