use std::collections::HashMap;
use std::iter::Iterator;


#[derive(Debug)]
struct Spiral {
    current: (i32, i32),
    dx: i32,
    dy: i32,
    remaining: i32,
    // how many remaining steps in current direction (denoted by dx, dy)
    steps: i32, // how many total steps in current state of the spiral

    rotate_map: HashMap<(i32, i32), (i32, i32)>, // map for the next dx, dy when rotating right
}

impl Spiral {
    fn new() -> Spiral {
        let rotate_vec = vec![
            ((1, 0), (0, -1)),
            ((0, -1), (-1, 0)),
            ((-1, 0), (0, 1)),
            ((0, 1), (1, 0)),
        ];
        Spiral {
            current: (0, 0),
            dx: 1,
            dy: 0,
            remaining: 1,
            steps: 1,
            rotate_map: rotate_vec.into_iter().collect(),
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
        } else {
            // if vertical increase steps
            if self.dy != 0 {
                self.steps += 1;
                self.remaining = self.steps
            }

            // rotate right
            (self.dx, self.dy) = self.rotate_map[&(self.dx, self.dy)]
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
        println!("test_spiral()");

        let s = Spiral::new();

        let points: Vec<(i32, i32)> = s.take(10).collect();
        let p0 = points[0];
        assert_eq!(p0, (0,0));

        let expected = vec!(
            [(0,0), (1,0),(1,-1),(0,-1),(-1,-1),(-1,0),
             (-1,1), (0,1),(1,1),(2,1),(2,0),(2,-1)]);

        // let zipped = points.zip(expected.into_iter());
        //
        // for (p, e) in zipped {
        //     assert_eq!(p, e);
        // }
    }
}