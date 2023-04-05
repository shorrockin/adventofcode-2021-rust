pub const NORTH: Direction = Direction(0, -1);
pub const NORTH_EAST: Direction = Direction(1, -1);
pub const EAST: Direction = Direction(1, 0);
pub const SOUTH_EAST: Direction = Direction(1, 1);
pub const SOUTH: Direction = Direction(0, 1);
pub const SOUTH_WEST: Direction = Direction(-1, 1);
pub const WEST: Direction = Direction(-1, 0);
pub const NORTH_WEST: Direction = Direction(-1, -1);

// simple structure to encode an x/y direction
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct Direction(pub i32, pub i32);

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Direction(x={}, y={})", self.0, self.1)
    }
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Direction({}, {})", self.0, self.1)
    }
}
