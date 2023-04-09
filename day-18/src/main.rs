use snailfish::Number;
mod snailfish;

const INPUT: &str = include_str!("input.txt");

fn input_to_snailfish(input: &str) -> Vec<Number> {
    input.lines().map(Number::new).collect()
}

fn part_one(input: &str) -> u32 {
    let addition = input_to_snailfish(input).into_iter().reduce(|left, right| {
        let mut sum = left + right;
        sum.reduce();
        sum
    });
    match addition {
        Some(result) => result.magnitude(),
        None => panic!("could not produce the addition"),
    }
}

fn part_two(input: &str) -> u32 {
    let numbers = input_to_snailfish(input);
    let mut max = 0;

    let mut update_max = |left: Number, right: Number| {
        let mut sum = left + right;
        sum.reduce();
        max = std::cmp::max(sum.magnitude(), max);
    };

    for left in numbers.iter() {
        for right in numbers.iter() {
            if left == right {
                continue;
            }

            update_max(left.clone(), right.clone());
            update_max(right.clone(), left.clone());
        }
    }

    max
}

fn main() {
    println!("Part 1: {}", part_one(INPUT));
    println!("Part 2: {}", part_two(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = include_str!("input.example.txt");

    #[test]
    fn test_example_input_part_1() {
        assert_eq!(4140, part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_example_input_part_2() {
        assert_eq!(3993, part_two(EXAMPLE_INPUT));
    }
}
