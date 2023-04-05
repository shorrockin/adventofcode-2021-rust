use crate::direction::Direction;

#[derive(Hash, Eq, PartialEq, Clone, Copy, PartialOrd)]
pub struct Coordinate(pub i32, pub i32);

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coordinate(x={}, y={})", self.0, self.1)
    }
}

impl std::fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coordinate({}, {})", self.0, self.1)
    }
}

impl core::ops::Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, direction: Direction) -> Self::Output {
        Coordinate(self.0 + direction.0, self.1 + direction.1)
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0 + self.1).cmp(&(other.0 + other.1))
    }
}

pub fn from_string(input: &str) -> Vec<(Coordinate, char)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y_coordinate, line)| {
            line.chars()
                .enumerate()
                .map(move |(x_coordinate, character)| {
                    (
                        Coordinate(x_coordinate as i32, y_coordinate as i32),
                        character,
                    )
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let test_data = "abc\nefg";
        let parsed = from_string(test_data);
        assert_eq!(6, parsed.len());

        let first = parsed.get(0).unwrap();
        assert_eq!(Coordinate(0, 0), first.0);
        assert_eq!('a', first.1);

        let second = parsed.get(4).unwrap();
        assert_eq!(Coordinate(1, 1), second.0);
        assert_eq!('f', second.1);
    }
}
