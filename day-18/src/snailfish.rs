use anyhow::{anyhow, Result};
use std::ops::{Index, IndexMut};

#[derive(Eq, PartialEq)]
enum ReduceAction {
    None,
    Explode(usize),
    Split(usize),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Number {
    elements: Vec<Element>,
}

impl std::ops::Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let mut left: Vec<Element> = self
            .elements
            .iter()
            .map(|e| Element::new(e.value, e.depth + 1))
            .collect();

        let right: Vec<Element> = rhs
            .elements
            .iter()
            .map(|e| Element::new(e.value, e.depth + 1))
            .collect();

        left.extend(right);
        Number { elements: left }
    }
}

impl Number {
    pub fn new(value: &str) -> Number {
        value
            .to_string()
            .try_into()
            .expect("should have been able to convert this string")
    }

    pub fn magnitude(&self) -> u32 {
        // feels like a smarter way exists, but this is what we'll do loop
        // we'll loop and find pairs of the same depth, reduce them and update values
        // and continue this reduction until we have no elements left
        let mut elements = self.elements.clone();
        let mut reduce_at: Option<usize> = None;

        while elements.len() != 1 {
            for (index, current) in elements.iter().enumerate() {
                if index + 1 == elements.len() {
                    panic!("we should not be testing the last element of the elements")
                }

                let next = elements.index(index + 1);
                if current.depth == next.depth {
                    reduce_at = Some(index);
                    break;
                }
            }

            match reduce_at {
                Some(index) => {
                    let current = *elements.index(index);
                    let next = *elements.index(index + 1);
                    let mut next_depth = 0;

                    // on the last reduction we don't want to reduce depth
                    if current.depth != 0 {
                        next_depth = current.depth - 1;
                    }

                    elements.drain(index..index + 2);
                    elements.insert(
                        index,
                        Element::new(current.value * 3 + next.value * 2, next_depth),
                    );
                }
                None => panic!("could not find anything to reduce"),
            }
        }

        elements.index(0).value
    }

    fn reduce_once(&mut self) -> bool {
        let mut action = ReduceAction::None;

        for (index, element) in self.elements.iter().enumerate() {
            if element.depth >= 4 {
                let next = self.elements.index(index + 1);
                if element.depth == next.depth {
                    action = ReduceAction::Explode(index);
                    break;
                }
            }
        }

        if action == ReduceAction::None {
            for (index, element) in self.elements.iter().enumerate() {
                if element.value >= 10 {
                    action = ReduceAction::Split(index);
                    break;
                }
            }
        }

        match action {
            ReduceAction::Explode(index) => {
                let exploding_left = *self.elements.index(index);
                let exploding_right = *self.elements.index(index + 1);

                assert!(exploding_left.depth == exploding_right.depth);

                if index != 0 {
                    let previous = self.elements.index_mut(index - 1);
                    previous.inc(exploding_left.value);
                }

                if index + 2 < self.elements.len() {
                    let next = self.elements.index_mut(index + 2);
                    next.inc(exploding_right.value);
                }

                self.elements.drain(index..index + 2);
                self.elements
                    .insert(index, Element::new(0, exploding_left.depth - 1));

                true
            }
            ReduceAction::Split(index) => {
                let split = *self.elements.index(index);
                let left_split = Element::new(split.value / 2, split.depth + 1);
                let right_split = Element::new(split.value - left_split.value, left_split.depth);

                self.elements.remove(index);
                self.elements.insert(index, right_split);
                self.elements.insert(index, left_split);

                true
            }
            ReduceAction::None => false,
        }
    }

    pub fn reduce(&mut self) {
        loop {
            match self.reduce_once() {
                false => break,
                true => (),
            }
        }
    }
}

impl TryInto<Number> for String {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Number> {
        let mut depth = 0;
        let mut elements = vec![];

        self.chars().try_for_each(|character| {
            match character {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => (),
                _ => match character.to_digit(10) {
                    Some(digit) => elements.push(Element::new(digit, depth - 1)),
                    None => {
                        return Err(anyhow!(
                            "unable to convert character to digit: '{}'",
                            character
                        ))
                    }
                },
            };
            Ok(())
        })?;

        Ok(Number { elements })
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Element {
    value: u32,
    depth: u32,
}

impl Element {
    pub fn new(value: u32, depth: u32) -> Element {
        Element { value, depth }
    }

    pub fn inc(&mut self, amount: u32) {
        self.value += amount
    }
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Element({} [depth: {}])", self.value, self.depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snailfish_addition() {
        let assert_add = |left: &str, right: &str, result: &str| {
            let sum = Number::new(left) + Number::new(right);
            assert_eq!(Number::new(result), sum);
        };

        assert_add("[1,2]", "[[3,4],5]", "[[1,2],[[3,4],5]]");
        assert_add(
            "[[[[4,3],4],4],[7,[[8,4],9]]]",
            "[1,1]",
            "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
        );
        assert_add(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
        );
    }

    #[test]
    fn test_snailfish_explosion() {
        let mut number = Number::new("[[[[[9,8],1],2],3],4]");
        number.reduce_once();
        assert_eq!(Number::new("[[[[0,9],2],3],4]"), number);

        number = Number::new("[7,[6,[5,[4,[3,2]]]]]");
        number.reduce_once();
        assert_eq!(Number::new("[7,[6,[5,[7,0]]]]"), number);

        number = Number::new("[[6,[5,[4,[3,2]]]],1]");
        number.reduce_once();
        assert_eq!(Number::new("[[6,[5,[7,0]]],3]"), number);

        number = Number::new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        number.reduce_once();
        assert_eq!(Number::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"), number);

        number = Number::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        number.reduce_once();
        assert_eq!(Number::new("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"), number);
    }

    #[test]
    fn test_snailfish_split() {
        let mut number = Number::new("[[1,9],1]");
        number.elements.index_mut(2).value = 11;
        number.reduce_once();
        assert_eq!(Number::new("[[1,9],[5,6]]"), number);

        number = Number::new("[[[[0,7],4],[[7,8],[0,1]]],[1,1]]");
        number.elements.index_mut(6).value = 13;
        number.reduce_once();
        assert_eq!(Number::new("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"), number);
    }

    #[test]
    fn test_snailfish_full_reduce() {
        let assert_reduce = |expected: &str, input: &str| {
            let mut number = Number::new(input);
            number.reduce();
            dbg!(&number);
            assert_eq!(Number::new(expected), number);
        };

        assert_reduce(
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
        );

        assert_reduce(
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
        );
    }

    #[test]
    fn test_magnitude_calculations() {
        let assert_mag = |e: u32, v: &str| assert_eq!(e, Number::new(v).magnitude());

        assert_mag(129, "[[9,1],[1,9]]");
        assert_mag(143, "[[1,2],[[3,4],5]]");
        assert_mag(1384, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_mag(445, "[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_mag(1137, "[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_mag(
            3488,
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        );
    }
}
