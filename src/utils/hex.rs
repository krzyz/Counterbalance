use std::hash::{Hash, Hasher};

use bevy::prelude::{IVec2, IVec3, Vec3};

fn manhattan_distance(a: &IVec3, b: &IVec3) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}

#[derive(Debug, Clone, Copy, Eq)]
pub enum Hex {
    Oddr(IVec2),
    Cube(IVec3),
}

impl Hash for Hex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_oddr().hash(state)
    }
}

impl PartialEq for Hex {
    fn eq(&self, other: &Self) -> bool {
        self.to_oddr().eq(&other.to_oddr())
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

impl Hex {
    pub fn from_oddr(pos: IVec2) -> Self {
        Self::Oddr(pos)
    }

    #[allow(dead_code)]
    pub fn from_cube(pos: IVec2) -> Self {
        Self::Cube(IVec3::new(pos.x, pos.y, -pos.x + pos.y))
    }

    #[allow(dead_code)]
    pub fn to_oddr(self) -> IVec2 {
        match self {
            Self::Oddr(pos) => pos,
            Self::Cube(pos) => {
                let x = pos.x - (pos.y + (pos.y & 1)) / 2;
                let y = pos.y;
                IVec2::new(x, y)
            }
        }
    }

    pub fn to_cube(self) -> IVec3 {
        match self {
            Self::Cube(pos) => pos,
            Self::Oddr(pos) => {
                let q = pos.x + (pos.y + (pos.y & 1)) / 2;
                let r = pos.y;
                IVec3::new(q, r, -q + r)
            }
        }
    }

    pub fn dist(&self, hex: Hex) -> i32 {
        manhattan_distance(&self.to_cube(), &hex.to_cube()) / 2
    }

    fn cube_lerp(a: Hex, b: Hex, t: f32) -> Vec3 {
        let a = a.to_cube();
        let b = b.to_cube();
        (
            lerp(a.x as f32, b.x as f32, t),
            lerp(a.y as f32, b.y as f32, t),
            lerp(a.z as f32, b.z as f32, t),
        )
            .into()
    }

    fn cube_round(frac: Vec3) -> Hex {
        let mut qrs = frac.round();
        let qrs_diff = (qrs - frac).abs();

        if qrs_diff.x > -qrs_diff.y && qrs_diff.x > qrs_diff.z {
            qrs.x = qrs.y - qrs.z;
        } else if -qrs_diff.y > qrs_diff.z {
            qrs.y = qrs.x + qrs.z;
        }

        Hex::from_cube((qrs.x as i32, qrs.y as i32).into())
    }

    pub fn line(self, end: Hex) -> Vec<Hex> {
        let n = self.dist(end);
        (0..=n)
            .map(|i| Self::cube_round(Self::cube_lerp(self, end, i as f32 / n as f32)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ODDR: [(i32, i32); 5] = [(0, 0), (0, 2), (2, 0), (1, 1), (2, 1)];
    const CUBE: [(i32, i32, i32); 5] = [(0, 0, 0), (1, 2, 1), (2, 0, -2), (2, 1, -1), (3, 1, -2)];
    // 1v1, 1v2, .. 1v5, 2v2, 2v3, .. 2v5, etc.
    const DISTS: [i32; 15] = [0, 2, 2, 2, 3, 0, 3, 2, 3, 0, 1, 1, 0, 1, 0];

    #[test]
    fn convert_from_oddr_to_cube() {
        let oddr = ODDR.into_iter().map(|p| p.into());
        let cube_expected = CUBE.into_iter().map(|p| p.into());
        for (oddr_val, cube_val_expected) in oddr.zip(cube_expected) {
            let cube_val = Hex::from_oddr(oddr_val).to_cube();
            assert_eq!(cube_val, cube_val_expected);
        }
    }

    #[test]
    fn convert_from_cube_to_oddr() {
        let cube = CUBE.into_iter().map(|p| (p.0, p.1).into());
        let oddr_expected = ODDR.into_iter().map(|p| p.into());
        for (cube_val, oddr_val_expected) in cube.zip(oddr_expected) {
            let oddr_val = Hex::from_cube(cube_val).to_oddr();
            assert_eq!(oddr_val, oddr_val_expected);
        }
    }

    #[test]
    fn distance_oddr() {
        let mut z = 0;
        for i in 0..ODDR.len() {
            for j in i..ODDR.len() {
                let hex1 = Hex::from_oddr(ODDR[i].into());
                let hex2 = Hex::from_oddr(ODDR[j].into());
                let dist_expected = DISTS[z];
                assert_eq!(
                    hex1.dist(hex2),
                    dist_expected,
                    "distance between {hex1:?} and {hex2:?} should be {dist_expected}"
                );
                z += 1;
            }
        }
    }

    #[test]
    fn distance_cube() {
        let mut z = 0;
        for i in 0..CUBE.len() {
            for j in i..CUBE.len() {
                let hex1 = CUBE[i];
                let hex1 = Hex::from_cube((hex1.0, hex1.1).into());
                let hex2 = CUBE[j];
                let hex2 = Hex::from_cube((hex2.0, hex2.1).into());
                let dist_expected = DISTS[z];
                assert_eq!(
                    hex1.dist(hex2),
                    dist_expected,
                    "distance between {hex1:?} and {hex2:?} should be {dist_expected}"
                );
                z += 1;
            }
        }
    }

    #[test]
    fn line_draw() {
        let start = Hex::from_oddr((0, 0).into());
        let end = Hex::from_oddr((5, 2).into());
        let line = start.line(end);

        let expected = [(0, 0), (1, 0), (1, 1), (2, 1), (3, 1), (4, 2), (5, 2)]
            .into_iter()
            .map(|p| Hex::from_oddr(p.into()))
            .collect::<Vec<_>>();

        assert_eq!(line, expected);
    }
}
