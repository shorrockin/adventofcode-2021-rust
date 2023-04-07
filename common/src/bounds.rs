use crate::coordinate::Coordinate;

pub struct Bounds {
    pub from: Coordinate,
    pub to: Coordinate,
}

impl Bounds {
    pub fn new(from: Coordinate, to: Coordinate) -> Bounds {
        Bounds { from, to }
    }

    pub fn contains(&self, other: &Coordinate) -> bool {
        self.from.0 <= other.0
            && self.from.1 <= other.1
            && self.to.0 >= other.0
            && self.to.1 >= other.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_within() {
        let bounds = Bounds::new(Coordinate(10, 10), Coordinate(20, 20));

        assert_eq!(true, bounds.contains(&Coordinate(15, 15)));
        assert_eq!(true, bounds.contains(&Coordinate(10, 10)));
        assert_eq!(true, bounds.contains(&Coordinate(20, 20)));
        assert_eq!(true, bounds.contains(&Coordinate(20, 19)));

        assert_eq!(false, bounds.contains(&Coordinate(9, 15)));
        assert_eq!(false, bounds.contains(&Coordinate(19, 21)));
    }
}
