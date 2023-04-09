use common::coordinate_3d::Coordinate3D;
use std::collections::HashMap;

pub struct Scanner {
    pub id: u32,
    pub beacons: Vec<Coordinate3D>,
    pub distances: HashMap<Coordinate3D, Vec<f32>>,
}
impl Scanner {
    pub fn new(id: u32) -> Scanner {
        Scanner {
            id,
            beacons: Vec::new(),
            distances: HashMap::new(),
        }
    }

    // updates all beacon distances, inserts the beacon along with their distances to other beacons
    pub fn insert(&mut self, new: Coordinate3D) {
        for beacon in &self.beacons {
            let distances = self.distances.get_mut(beacon).unwrap();
            distances.push(beacon.distance(&new));
        }

        self.beacons.push(new.clone());

        let mut distances = Vec::new();
        for beacon in &self.beacons {
            match new.distance(beacon) {
                distance if distance == 0.0 => continue,
                distance => distances.push(distance),
            }
        }

        self.distances.insert(new, distances);
    }

    pub fn parse(input: &str) -> Vec<Scanner> {
        let chunks: Vec<_> = input.split("\n\n").collect();

        chunks
            .iter()
            .map(|chunk| {
                let mut lines = chunk.lines();
                let id = lines
                    .next()
                    .unwrap()
                    .split(' ')
                    .nth(2)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                let mut scanner = Scanner::new(id);
                for line in lines {
                    scanner.insert(line.parse::<Coordinate3D>().unwrap());
                }
                scanner
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("input.example.txt");

    #[test]
    fn parse_scanners() {
        let scanners = Scanner::parse(EXAMPLE_INPUT);
        assert_eq!(5, scanners.len());

        let scanner = &scanners[0];
        assert_eq!(0, scanner.id);
        assert_eq!(25, scanner.beacons.len());

        assert_eq!(Coordinate3D(404, -588, -901), scanner.beacons[0]);
        assert_eq!(Coordinate3D(528, -643, 409), scanner.beacons[1]);
        assert_eq!(Coordinate3D(-838, 591, 734), scanner.beacons[2]);

        let distances = scanner
            .distances
            .get(&Coordinate3D(404, -588, -901))
            .unwrap();
        assert_eq!(distances.len(), 24);

        let scanner = &scanners[1];
        assert_eq!(1, scanner.id);
        assert_eq!(25, scanner.beacons.len());

        assert_eq!(Coordinate3D(686, 422, 578), scanner.beacons[0]);
        assert_eq!(Coordinate3D(605, 423, 415), scanner.beacons[1]);
        assert_eq!(Coordinate3D(515, 917, -361), scanner.beacons[2]);
    }
}
