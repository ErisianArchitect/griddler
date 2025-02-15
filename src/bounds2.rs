
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bounds2 {
    pub min: (i32, i32),
    pub max: (i32, i32),
}

impl Bounds2 {
    pub const fn new(min: (i32, i32), max: (i32, i32)) -> Self {
        Self {
            min,
            max,
        }
    }

    #[track_caller]
    pub fn from_points(points: &[(i32, i32)]) -> Self {
        assert!(points.len() != 0, "No points provided.");
        let (mut min_x, mut min_y) = points[0];
        let (mut max_x, mut max_y) = (min_x, min_y);
        for &(x, y) in &points[1..] {
            if x < min_x {
                min_x = x;
            } else if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            } else if y > max_y {
                max_y = y;
            }
        }
        Self::new(
            (min_x, min_y),
            (max_x, max_y),
        )
    }
}

#[should_panic]
#[test]
fn from_points_test() {
    let bounds = Bounds2::from_points(&[]);
}