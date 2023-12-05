use std::collections::HashMap;
use std::iter::Iterator;

#[derive(Debug)]
pub struct Spiral {
    current: (i32, i32),
    dx: i32,
    dy: i32,
    remaining: i32,
    // how many remaining steps in current direction (denoted by dx, dy)
    arm: i32, // how many total steps in current state of the spiral
    turns: i32, // total turns
    rotate_map: HashMap<(i32, i32), (i32, i32)>, // map for the next dx, dy when rotating right
}

impl Spiral {
    pub fn new() -> Spiral {
        let rotate_vec = vec![
            ((1, 0), (0, -1)),
            ((0, -1), (-1, 0)),
            ((-1, 0), (0, 1)),
            ((0, 1), (1, 0)),
        ];
        Spiral {
            current: (0, 0),
            dx: 0,
            dy: 1,
            remaining: 1,
            arm: 0,
            rotate_map: rotate_vec.into_iter().collect(),
            turns: 0,
        }
    }
}


// Implement the Iterator trait for Spiral
impl Iterator for Spiral {
    type Item = (i32, i32);

    // Define the next() method, which produces the next element in the sequence
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        // Increment the current value and return it
        if self.remaining > 0 {
            self.remaining -= 1
        }
        if self.remaining == 0 {
            // rotate right
            (self.dx, self.dy) = self.rotate_map[&(self.dx, self.dy)];

            // every two turns of the spiral increase steps
            if self.turns % 2 == 0 {
                self.arm += 1;
            }
            self.remaining = self.arm;
            self.turns += 1; // how many time did the spiral turn
        }

        self.current.0 += self.dx;
        self.current.1 += self.dy;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::spiral::Spiral;

    #[test]
    fn test_spiral() {
        let s = Spiral::new();
        let points: Vec<(i32, i32)> = s.take(12).collect();
        let p0 = points[0];
        assert_eq!(p0, (0,0));

        let expected = vec!(
            (0,0), (1,0),(1,-1),(0,-1),(-1,-1),(-1,0),
            (-1,1), (0,1),(1,1),(2,1),(2,0),(2,-1)
        );

        assert_eq!(points, expected);
        for i in 0..points.len() {
            assert_eq!(&points[i], &expected[i]);
        }
    }
}