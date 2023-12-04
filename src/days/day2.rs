use crate::helpers::get_data_as_array;
use std::process::exit;

#[derive(Debug)]
struct Game {
    index: usize,
    subsets: Vec<GameSubset>,
    max_values: (usize, usize, usize),
}

impl From<String> for Game {
    /// String should be in the format "Game {index}: {subset 1}; {subset 2}; ..."
    fn from(mut string: String) -> Game {
        if string.starts_with("\"") && string.ends_with("\"") {
            // Trim the escaped quotes from both ends
            string = string[1..string.len() - 1].to_string();
        }
        let mut parts: Vec<&str> = string.split(": ").collect();
        let index = parts
            .remove(0)
            .strip_prefix("Game ")
            .expect("Game prefix strip to work")
            .parse::<usize>()
            .expect("Game index parse to work");
        let subsets = parts
            .pop()
            .expect("Subset string portion to exist")
            .split("; ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|string| GameSubset::from(string.to_string()))
            .collect::<Vec<GameSubset>>();
        Game {
            index,
            subsets,
            max_values: (12, 13, 14),
        }
    }
}

impl Game {
    fn impossible(&self) -> bool {
        for subset in &self.subsets {
            if subset.impossible() {
                return true;
            }
        }
        false
    }

    fn set_max_values(&mut self, red_max: usize, green_max: usize, blue_max: usize) {
        self.max_values = (red_max, green_max, blue_max);
        for subset in &mut self.subsets {
            subset.set_max_values(red_max, green_max, blue_max);
        }
    }

    fn get_fewest_cubes_needed(&self) -> (usize, usize, usize) {
        let (mut red_max, mut green_max, mut blue_max) = (0, 0, 0);
        for subset in &self.subsets {
            if red_max < subset.red {
                red_max = subset.red;
            }
            if green_max < subset.green {
                green_max = subset.green;
            }
            if blue_max < subset.blue {
                blue_max = subset.blue;
            }
        }
        (red_max, green_max, blue_max)
    }

    fn get_power_of_minimum_set_of_cubes(&self) -> usize {
        let (red_max, green_max, blue_max) = self.get_fewest_cubes_needed();
        red_max * green_max * blue_max
    }
}

#[derive(Debug)]
struct GameSubset {
    blue: usize,
    red: usize,
    green: usize,
    max_values: (usize, usize, usize),
}

impl From<String> for GameSubset {
    /// String should be a comma separated list of counts for each colour
    fn from(string: String) -> GameSubset {
        let parts: Vec<&str> = string.split(", ").collect();
        let (mut blue, mut red, mut green) = (0, 0, 0);
        for part in parts {
            match part {
                x if x.contains("blue") => {
                    blue += x
                        .strip_suffix(" blue")
                        .expect("blue prefix to exist")
                        .parse::<usize>()
                        .expect("parse to succeed");
                }
                x if x.contains("green") => {
                    green += x
                        .strip_suffix(" green")
                        .expect("green prefix to exist")
                        .parse::<usize>()
                        .expect("parse to succeed");
                }
                x if x.contains("red") => {
                    red += x
                        .strip_suffix(" red")
                        .expect("red prefix to exist")
                        .parse::<usize>()
                        .expect("parse to succeed");
                }
                _ => panic!("this should never happen"),
            }
        }
        GameSubset {
            blue,
            red,
            green,
            max_values: (12, 13, 14),
        }
    }
}

impl GameSubset {
    fn impossible(&self) -> bool {
        self.red > self.max_values.0
            || self.green > self.max_values.1
            || self.blue > self.max_values.2
    }

    fn set_max_values(&mut self, red_max: usize, green_max: usize, blue_max: usize) {
        self.max_values = (red_max, green_max, blue_max);
    }
}

#[allow(unreachable_code)]
pub fn run() -> String {
    let data = get_data_as_array("./data/day2.json".to_string());
    let mut p1_total = 0;
    let mut p2_total = 0;
    for string in data {
        let mut game = Game::from(string);
        p2_total += game.get_power_of_minimum_set_of_cubes();
        game.set_max_values(12, 13, 14);
        if game.impossible() {
            continue;
        }
        p1_total += game.index;
    }
    return format!("P1: {}, P2: {}", p1_total, p2_total);
}
