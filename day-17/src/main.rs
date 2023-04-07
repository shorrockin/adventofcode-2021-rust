mod probe;

use common::bounds::Bounds;
use common::coordinate::Coordinate;
use common::direction::Direction;
use probe::Probe;
use regex::Regex;
use std::cmp::max;

fn parse(value: &str) -> Bounds {
    let re = Regex::new(r"x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)").unwrap();
    let captures = re.captures_iter(value).next().unwrap();

    let x_start: i32 = captures[1].parse().unwrap();
    let x_end: i32 = captures[2].parse().unwrap();
    let y_start: i32 = captures[3].parse().unwrap();
    let y_end: i32 = captures[4].parse().unwrap();

    Bounds {
        from: Coordinate(x_start, y_start),
        to: Coordinate(x_end, y_end),
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ShotResult {
    Hit(Coordinate, i32),
    Undershot(Coordinate),
    Overshot(Coordinate),
}

fn shoot_probe(velocity: Direction, target: &Bounds) -> ShotResult {
    let mut probe = Probe::new(velocity);
    let mut max_height = 0;
    loop {
        let position = probe.step();
        max_height = max(max_height, position.1);

        if target.contains(&position) {
            return ShotResult::Hit(position, max_height);
        } else if is_overshot(&position, target) {
            return ShotResult::Overshot(position);
        } else if is_undershot(&position, target) {
            return ShotResult::Undershot(position);
        }
    }
}

fn run_tests(target: &Bounds, test_range: i32) -> (i32, u32) {
    // this is dumb, but rust is fast, revisit later.... maybe
    let mut best: i32 = 0;
    let mut hits: u32 = 0;
    for x in 1..test_range {
        for y in -test_range..test_range {
            if let ShotResult::Hit(_, height) = shoot_probe(Direction(x, y), target) {
                best = max(best, height);
                hits += 1
            }
        }
    }
    (best, hits)
}

fn is_overshot(position: &Coordinate, target: &Bounds) -> bool {
    position.0 > target.from.0 && position.0 > target.to.0
}

fn is_undershot(position: &Coordinate, target: &Bounds) -> bool {
    position.1 < target.to.1 && position.1 < target.from.1
}

fn main() {
    let bounds = parse("target area: x=253..280, y=-73..-46");
    println!("Part 1: {}", run_tests(&bounds, 1000).0);
    println!("Part 2: {}", run_tests(&bounds, 1000).1);
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_bounds_parsing() {
        let bounds = parse(EXAMPLE_INPUT);
        assert_eq!(Coordinate(20, -10), bounds.from);
        assert_eq!(Coordinate(30, -5), bounds.to);
    }

    #[test]
    fn test_example_input() {
        let bounds = parse(EXAMPLE_INPUT);
        assert_eq!(
            ShotResult::Hit(Coordinate(21, -9), 6),
            shoot_probe(Direction(6, 3), &bounds)
        );
        assert_eq!(
            ShotResult::Hit(Coordinate(28, -7), 3),
            shoot_probe(Direction(7, 2), &bounds)
        );
        assert_eq!(
            ShotResult::Hit(Coordinate(30, -6), 0),
            shoot_probe(Direction(9, 0), &bounds)
        );
        assert_eq!(
            ShotResult::Hit(Coordinate(21, -10), 45),
            shoot_probe(Direction(6, 9), &bounds)
        );
        assert_eq!(
            ShotResult::Overshot(Coordinate(33, -9)),
            shoot_probe(Direction(33, -9), &bounds)
        );
        assert_eq!(
            ShotResult::Overshot(Coordinate(34, -6)),
            shoot_probe(Direction(10, 0), &bounds)
        );
        assert_eq!(
            ShotResult::Undershot(Coordinate(1, -15)),
            shoot_probe(Direction(1, 0), &bounds)
        );
    }

    #[test]
    fn test_solution_solver() {
        let bounds = parse(EXAMPLE_INPUT);
        let result = run_tests(&bounds, 31);
        assert_eq!(45, result.0);
        assert_eq!(112, result.1);
    }
}
