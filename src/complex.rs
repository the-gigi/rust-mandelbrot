use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Complex {
    r: f64,
    i: f64,
}

impl Complex {
    pub fn new(r: f64, i: f64) -> Complex {
        Complex { r, i }
    }

    pub fn abs_sq(self) -> f64 {
        //self.r.saturating_mul(self.r).saturating_sub(self.i.saturating_mul(self.i))
        self.r * self.r + self.i * self.i
    }
}



impl ops::Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Complex {
            r: self.r + other.r,
            i: self.i + other.i
        }
    }
}

impl ops::Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Complex {
            // r: self.r.saturating_mul(other.r).saturating_sub(self.i.saturating_mul(other.i)),
            // i: self.r .saturating_mul(other.i).saturating_add(other.r.saturating_mul(self.i))
            r: self.r * other.r - self.i * other.i,
            i: self.r * other.i + self.i * other.r
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::complex::Complex;

    #[test]
    fn test_create() {
        let z = Complex::new(5.0, 3.0);
        assert_eq!(z.r, 5.0);
        assert_eq!(z.i, 3.0);
    }

    #[test]
    fn test_abs_sq() {
        let z = Complex::new(2.0, -3.0);
        assert_eq!(z.abs_sq(), 13.0);
    }
    #[test]
    fn test_arithmetic() {
        let z1 = Complex::new(5.0, 3.0);
        let z2 = Complex::new(4.0, 2.0);

        let sum = z1.clone() + z2.clone();
        let product = z1.clone() * z2.clone();
        assert_eq!(sum, Complex::new(9.0, 5.0));
        assert_eq!(product, Complex::new(14.0, 22.0))
    }
}