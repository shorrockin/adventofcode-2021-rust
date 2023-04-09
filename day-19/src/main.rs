use crate::scanner::Scanner;

mod scanner;

const INPUT: &str = include_str!("input.txt");
const INTERSECTION_THRESHOLD: i32 = 2;

fn part_one(scanners: &Vec<Scanner>) -> i32 {
    let scanner = reduce(scanners);
    scanner.beacons.len() as i32
}

fn part_two(_scanners: &Vec<Scanner>) -> i32 {
    // returns the max distance in our scanner.distances hashmap
    todo!()
}

fn reduce(scanners: &Vec<Scanner>) -> Scanner {
    let mut root = Scanner::new(0);

    // loop through all scanners in our parsed input
    for scanner in scanners {
        for beacon in &scanner.beacons {
            let existing_distances = scanner.distances.get(beacon).unwrap();
            if max_intersections(&root, existing_distances) < INTERSECTION_THRESHOLD {
                root.insert(*beacon);
            }
        }
    }

    root
}

fn max_intersections(root: &Scanner, distances: &Vec<f32>) -> i32 {
    let mut max = 0;
    for beacon in &root.beacons {
        let beacon_distance = root.distances.get(beacon).unwrap();
        let overlap = intersections(beacon_distance, distances);
        if overlap > max {
            max = overlap;
        }
    }
    max
}

fn intersections(left: &Vec<f32>, right: &Vec<f32>) -> i32 {
    let mut count = 0;
    for left in left.iter() {
        if right.contains(left) {
            count += 1;
        }
    }
    count
}

fn main() {
    let scanners = scanner::Scanner::parse(INPUT);

    println!("Part 1: {}", part_one(&scanners));
    println!("Part 2: {}", part_two(&scanners));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("input.example.txt");

    #[test]
    fn test_intersections() {
        assert_eq!(1, intersections(&vec![1.0], &vec![1.0]));
        assert_eq!(0, intersections(&vec![1.0], &vec![2.0]));
        assert_eq!(2, intersections(&vec![1.0, 2.0], &vec![1.0, 2.0]));
        assert_eq!(1, intersections(&vec![1.0, 2.0], &vec![1.0, 3.0]));
        assert_eq!(0, intersections(&vec![1.0, 2.0], &vec![3.0, 4.0]));
        assert_eq!(1, intersections(&vec![1.0, 2.0], &vec![3.0, 4.0, 1.0]));
        assert_eq!(2, intersections(&vec![4.0, 5.0, 1.0, 2.0], &vec![4.0, 1.0]))
    }

    #[test]
    fn example_input_part_one() {
        let scanners = Scanner::parse(EXAMPLE_INPUT);
        assert_eq!(79, part_one(&scanners));
    }

    #[test]
    fn example_input_part_two() {
        //let scanners = Scanner::parse(EXAMPLE_INPUT);
        //assert_eq!(3621, part_two(&scanners));
    }
}
