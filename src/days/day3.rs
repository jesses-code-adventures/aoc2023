use crate::helpers::get_data_as_array;
use std::collections::HashSet;

fn is_period(c: char) -> bool {
    c == '.'
}

fn is_symbol(c: char) -> bool {
    !is_period(c) && !c.is_numeric()
}

fn get_surrounding_coordinates(
    x_start: usize,
    x_end: usize,
    y_start: usize,
    y_end: usize,
    schematic_height: usize,
    schematic_width: usize,
) -> Vec<(usize, usize)> {
    let mut coords: Vec<(usize, usize)> = vec![];
    let include_left = x_start > 0;
    let include_above = y_start > 0;
    let include_below = y_end == 0 || y_end < schematic_height - 1;
    let include_right = x_end == 0 || x_end < schematic_width - 1;
    let mut x_range_start = x_start;
    if include_left {
        x_range_start -= 1
    };
    let mut y_range_start = y_start;
    if include_above {
        y_range_start -= 1
    };
    let mut x_range_end = x_end;
    if include_right {
        x_range_end += 1
    };
    let mut y_range_end = y_end;
    if include_below {
        y_range_end += 1
    };
    for y in y_range_start..=y_range_end {
        for x in x_range_start..=x_range_end {
            if x_start <= x && x <= x_end && y_start <= y && y <= y_end {
                continue;
            }
            coords.push((x, y));
        }
    }
    coords
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct PossiblePartLocation {
    array_index: usize,
    start_index: usize,
    end_index: usize,
    value: usize,
    verified: bool,
}

impl PossiblePartLocation {
    /// Returns coordinates to check for symbols as a Vec of (x, y) tuples
    fn get_check_coordinates(
        &self,
        schematic_width: usize,
        schematic_height: usize,
    ) -> Vec<(usize, usize)> {
        get_surrounding_coordinates(
            self.start_index,
            self.end_index,
            self.array_index,
            self.array_index,
            schematic_height,
            schematic_width,
        )
    }

    fn is_at_coordinate(&self, x: usize, y: usize) -> bool {
        self.start_index <= x && x <= self.end_index && y == self.array_index
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Symbol {
    character: char,
    array_index: usize,
    character_index: usize,
}

#[derive(Debug)]
struct Schematic {
    possible_part_locations: Vec<PossiblePartLocation>,
    symbols: Vec<Symbol>,
    width: usize,
    height: usize,
}

impl Schematic {
    fn from_data(mut data: Vec<String>) -> Schematic {
        let mut symbols: Vec<Symbol> = vec![];
        let mut possible_part_locations: Vec<PossiblePartLocation> = vec![];
        let width = data.first().expect("data to have rows").len() - 2;
        let mut height = 0;
        for (line_index, line) in data.iter_mut().enumerate() {
            if line.starts_with('"') {
                *line = line.strip_prefix('"').unwrap().to_string();
            }
            if line.ends_with('"') {
                *line = line.strip_suffix('"').unwrap().to_string();
            }
            let mut possible_part_started = false;
            let mut possible_part_start: Option<usize> = None;
            let mut part_value_chars = String::new();
            for (char_index, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    if !possible_part_started {
                        possible_part_started = true;
                        possible_part_start = Some(char_index)
                    }
                    part_value_chars.push(c);
                    if char_index + 1 == width {
                        possible_part_locations.push(PossiblePartLocation {
                            array_index: line_index,
                            start_index: possible_part_start.expect("possible part start to exist"),
                            end_index: char_index - 1,
                            verified: false,
                            value: part_value_chars
                                .parse::<usize>()
                                .expect("value parsing to succeeed"),
                        });
                        possible_part_started = false;
                        possible_part_start = None;
                        part_value_chars.clear();
                    }
                    continue;
                }
                if is_period(c) {
                    if possible_part_start.is_some() {
                        possible_part_locations.push(PossiblePartLocation {
                            array_index: line_index,
                            start_index: possible_part_start.expect("possible part start to exist"),
                            end_index: char_index - 1,
                            verified: false,
                            value: part_value_chars
                                .parse::<usize>()
                                .expect("value parsing to succeeed"),
                        });
                        possible_part_started = false;
                        possible_part_start = None;
                        part_value_chars.clear();
                    }
                    continue;
                }
                if is_symbol(c) {
                    if possible_part_start.is_some() {
                        possible_part_locations.push(PossiblePartLocation {
                            array_index: line_index,
                            start_index: possible_part_start.expect("possible part start to exist"),
                            end_index: char_index - 1,
                            verified: false,
                            value: part_value_chars
                                .parse::<usize>()
                                .expect("value parsing to succeeed"),
                        });
                        possible_part_started = false;
                        possible_part_start = None;
                        part_value_chars.clear();
                    }
                    symbols.push(Symbol {
                        character: c,
                        array_index: line_index,
                        character_index: char_index,
                    });
                    continue;
                }
            }
            height = line_index + 1;
        }
        Schematic {
            possible_part_locations,
            symbols,
            width,
            height,
        }
    }

    fn mark_real_parts(&mut self) {
        for part in self.possible_part_locations.iter_mut() {
            let check_coords = part.get_check_coordinates(self.width, self.height);
            for (x, y) in check_coords {
                let mut found = false;
                for symbol in &self.symbols {
                    if symbol.array_index == y && symbol.character_index == x {
                        found = true;
                        part.verified = true;
                        break;
                    }
                }
                if found {
                    break;
                };
            }
        }
    }

    fn get_surrounding_parts(&self, symbol: &Symbol) -> HashSet<&PossiblePartLocation> {
        let coords = get_surrounding_coordinates(
            symbol.character_index,
            symbol.character_index,
            symbol.array_index,
            symbol.array_index,
            self.height,
            self.width,
        );
        let mut parts: HashSet<&PossiblePartLocation> = HashSet::new();
        for (x, y) in coords.into_iter() {
            for part in &self.possible_part_locations {
                if part.is_at_coordinate(x, y) {
                    parts.insert(part);
                    break;
                }
            }
        }
        parts
    }

    fn get_sum_of_real_parts(&self) -> usize {
        let mut total = 0;
        for part in &self
            .possible_part_locations
            .clone()
            .into_iter()
            .filter(|x| x.verified == true)
            .collect::<Vec<PossiblePartLocation>>()
        {
            total += part.value
        }
        total
    }

    fn get_gears(&self) -> HashSet<(&PossiblePartLocation, &PossiblePartLocation)> {
        let mut gears: HashSet<(&PossiblePartLocation, &PossiblePartLocation)> = HashSet::new();
        for symbol in &self.symbols {
            let mut surrounding_parts = self
                .get_surrounding_parts(symbol)
                .drain()
                .collect::<Vec<&PossiblePartLocation>>();
            if symbol.character == '*' && surrounding_parts.len() == 2 {
                gears.insert((
                    surrounding_parts.pop().unwrap(),
                    surrounding_parts.pop().unwrap(),
                ));
            }
        }
        gears
    }

    fn get_sum_of_gear_ratios(&self) -> usize {
        let gears = self.get_gears();
        let mut total = 0;
        for (gear1, gear2) in gears {
            total += gear1.value * gear2.value;
        }
        total
    }
}

#[allow(unreachable_code)]
pub fn run() -> String {
    let mut answer_1 = String::new();
    let mut answer_2 = String::new();
    let data = get_data_as_array("./data/day3.json".to_string());
    let mut schematic = Schematic::from_data(data);
    schematic.mark_real_parts();
    answer_1.push_str(schematic.get_sum_of_real_parts().to_string().as_str());
    answer_2.push_str(schematic.get_sum_of_gear_ratios().to_string().as_str());
    format!("P1: {}, P2: {}", answer_1, answer_2)
}
