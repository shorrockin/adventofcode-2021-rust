use common::coordinate::Coordinate;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

use common::direction::{EAST, NORTH, SOUTH, WEST};

static INPUT: &str = include_str!("input.txt");

enum Part {
    One,
    Two,
}

fn parse(input: &str, part: Part) -> HashMap<Coordinate, u32> {
    let initial_map = common::coordinate::from_string(input)
        .into_iter()
        .map(|(coordinate, character)| (coordinate, character.to_digit(10).unwrap()))
        .collect();

    match part {
        Part::One => initial_map,
        Part::Two => {
            let max_coord = initial_map.keys().max().unwrap();
            let width = max_coord.0 + 1; // assumes square grid
            let width_i32 = width as i32;
            let mut out: HashMap<Coordinate, u32> = HashMap::with_capacity(initial_map.len() * 25);

            // certainly could do this synthetically and not build out all the other
            // but performance seems fine for this amount of data.
            for (coordinate, base_risk) in initial_map {
                for x in 0..5 {
                    for y in 0..5 {
                        let mut risk = base_risk + (x as u32) + (y as u32);
                        if risk >= 10 {
                            risk = (risk % 10) + 1;
                        }

                        out.insert(
                            Coordinate(
                                coordinate.0 + (width_i32 * x),
                                coordinate.1 + (width_i32 * y),
                            ),
                            risk,
                        );
                    }
                }
            }
            out
        }
    }
}

fn solve(input: &str, part: Part) -> u32 {
    let grid = parse(input, part);
    let start = Coordinate(0, 0);
    let end = *grid.keys().max().unwrap();

    let neigbors = |coordinate: &Coordinate| {
        [
            *coordinate + NORTH,
            *coordinate + EAST,
            *coordinate + SOUTH,
            *coordinate + WEST,
        ]
        .into_iter()
        .filter(|coordinate| grid.contains_key(coordinate))
        .map(|coordinate| (coordinate, *grid.get(&coordinate).unwrap()))
        .collect::<Vec<(Coordinate, u32)>>()
    };

    let finished = |coordinate: &Coordinate| *coordinate == end;

    match dijkstra(&start, neigbors, finished) {
        Some((_, cost)) => cost,
        _ => panic!("could not determine path"),
    }
}

fn main() {
    println!("Part One: {}", solve(INPUT, Part::One));
    println!("Part Two: {}", solve(INPUT, Part::Two));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_EXAMPLE: &str = include_str!("input.example.txt");

    #[test]
    fn test_grid_parsing() {
        let coordinates = parse(INPUT_EXAMPLE, Part::One);
        assert_eq!(1, *coordinates.get(&Coordinate(0, 0)).unwrap());
        assert_eq!(8, *coordinates.get(&Coordinate(2, 1)).unwrap());
    }

    #[test]
    fn test_part_one_example_input() {
        assert_eq!(40, solve(INPUT_EXAMPLE, Part::One));
    }

    #[test]
    fn test_part_two_example_input() {
        assert_eq!(315, solve(INPUT_EXAMPLE, Part::Two));
    }
}
