use common::coordinate::Coordinate;
use common::direction::Direction;

pub struct Probe {
    position: Coordinate,
    velocity: Direction,
    step_count: usize,
}

impl Probe {
    pub fn new(velocity: Direction) -> Probe {
        Probe {
            velocity,
            position: Coordinate(0, 0),
            step_count: 0,
        }
    }

    pub fn step(&mut self) -> Coordinate {
        self.position = self.position + self.velocity;
        self.step_count += 1;
        self.velocity = Direction(
            match self.velocity.0 {
                x if x > 0 => x - 1,
                x if x < 0 => x + 1,
                _ => 0,
            },
            self.velocity.1 - 1,
        );

        self.position
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_probe_step() {
        let mut probe = Probe::new(Direction(7, 2));

        assert_eq!(Coordinate(7, 2), probe.step());
        assert_eq!(Coordinate(13, 3), probe.step());
        assert_eq!(Coordinate(18, 3), probe.step());
        assert_eq!(Coordinate(22, 2), probe.step());
        assert_eq!(Coordinate(25, 0), probe.step());
        assert_eq!(Coordinate(27, -3), probe.step());
        assert_eq!(Coordinate(28, -7), probe.step());
        assert_eq!(Coordinate(28, -12), probe.step());
        assert_eq!(Coordinate(28, -18), probe.step());
    }
}
