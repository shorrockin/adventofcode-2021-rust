use std::str::FromStr;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Coordinate3D(pub i32, pub i32, pub i32);

impl std::fmt::Display for Coordinate3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coordinate3D(x={},y={},z={})", self.0, self.1, self.2)
    }
}

impl std::fmt::Debug for Coordinate3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coordinate3D({},{},{})", self.0, self.1, self.2)
    }
}

impl PartialOrd for Coordinate3D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.0, self.1, self.2).partial_cmp(&(other.0, other.1, other.2))
    }
}

impl Ord for Coordinate3D {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0, self.1, self.2).cmp(&(other.0, other.1, other.2))
    }
}

impl Coordinate3D {
    pub fn distance(&self, other: &Coordinate3D) -> f32 {
        let dx = (other.0 - self.0) as f32;
        let dy = (other.1 - self.1) as f32;
        let dz = (other.2 - self.2) as f32;
        f32::sqrt(dx * dx + dy * dy + dz * dz)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCoordinate3DError;

// converts a string like 1,2,3 to a coordinate
impl FromStr for Coordinate3D {
    type Err = ParseCoordinate3DError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(',').map(|s| s.trim()).collect();
        if parts.len() != 3 {
            return Err(ParseCoordinate3DError);
        }

        let x = parts
            .first()
            .ok_or(ParseCoordinate3DError)?
            .parse::<i32>()
            .map_err(|_| ParseCoordinate3DError)?;
        let y = parts
            .get(1)
            .ok_or(ParseCoordinate3DError)?
            .parse::<i32>()
            .map_err(|_| ParseCoordinate3DError)?;
        let z = parts
            .get(2)
            .ok_or(ParseCoordinate3DError)?
            .parse::<i32>()
            .map_err(|_| ParseCoordinate3DError)?;

        Ok(Coordinate3D(x, y, z))
    }
}

//#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
//enum RotationElement {
//    X(i32),
//    Y(i32),
//    Z(i32),
//}
//impl RotationElement {
//    fn invert(&self) -> RotationElement {
//        match self {
//            X(neg) => X(neg * -1),
//            Y(neg) => Y(neg * -1),
//            Z(neg) => Z(neg * -1),
//        }
//    }
//
//    fn sign(&self, sign: i32) -> RotationElement {
//        match self {
//            X(neg) => X(neg * sign),
//            Y(neg) => Y(neg * sign),
//            Z(neg) => Z(neg * sign),
//        }
//    }
//
//    fn apply(&self, target: &Coordinate3D) -> i32 {
//        match self {
//            X(sign) => target.0 * sign,
//            Y(sign) => target.1 * sign,
//            Z(sign) => target.2 * sign,
//        }
//    }
//}
//
//use RotationElement::*;
//
//#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
//pub struct Rotation {
//    x: RotationElement,
//    y: RotationElement,
//    z: RotationElement,
//}
//
//impl Rotation {
//    fn new(x: RotationElement, y: RotationElement, z: RotationElement) -> Rotation {
//        Rotation { x, y, z }
//    }
//
//    pub fn apply(&self, target: &Coordinate3D) -> Coordinate3D {
//        let x = self.x.apply(target);
//        let y = self.y.apply(target);
//        let z = self.z.apply(target);
//
//        Coordinate3D(x, y, z)
//    }
//
//    pub fn rotations() -> HashSet<Rotation> {
//        let mut rotations = HashSet::new();
//
//        let x = X(1);
//        let y = Y(1);
//        let z = Z(1);
//
//        for &s in &[1, -1] {
//            // Rotations around the x-axis
//            rotations.insert(Rotation::new(x, y.sign(s), z.sign(s)));
//            rotations.insert(Rotation::new(x, z.sign(s), y.invert().sign(s)));
//
//            // Rotations around the y-axis
//            rotations.insert(Rotation::new(z.sign(s), y, x.sign(s)));
//            rotations.insert(Rotation::new(y.invert().sign(s), y, z.sign(s)));
//
//            // Rotations around the z-axis
//            rotations.insert(Rotation::new(y.sign(s), x.invert().sign(s), z));
//            rotations.insert(Rotation::new(z.invert().sign(s), y.sign(s), z));
//
//            // Rotations around the diagonals
//            rotations.insert(Rotation::new(y, z, x));
//            rotations.insert(Rotation::new(z, x, y));
//            rotations.insert(Rotation::new(x.invert(), z.invert(), y.invert()));
//            rotations.insert(Rotation::new(y.invert(), x.invert(), z.invert()));
//        }
//
//        rotations
//    }
//}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_calculation() {
        let a = Coordinate3D(10, 10, 10);
        let b = Coordinate3D(20, 20, 20);
        let distance = a.distance(&b);

        assert_eq!(17.320509, distance);
    }

    #[test]
    fn from_str() {
        assert_eq!(
            Coordinate3D(1, 2, 3),
            Coordinate3D::from_str("1,2,3").unwrap()
        );
        assert_eq!(
            Coordinate3D(-1, 2, 4),
            Coordinate3D::from_str("-1, 2,4").unwrap()
        );
    }

    //#[test]
    //fn rotations() {
    //    let rotators = Rotation::rotations();
    //    rotators.iter().for_each(|r| {
    //        dbg!(&r);
    //    });

    //    assert_eq!(24, rotators.len());

    //    let base = Coordinate3D(1, 2, 3);
    //    let rotations: HashSet<_> = rotators.iter().map(|r| r.apply(&base)).collect();

    //    assert_eq!(24, rotations.len());
    //}
}
