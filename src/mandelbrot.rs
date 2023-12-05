use crate::complex::Complex;


const ITERATIONS: i32 = 200;


/// This function checks if a complex number belongs to the mandelbrot set.
///
/// # Arguments
///
/// * `n` - An i32 integer that will be checked.
///
/// # Examples
pub fn is_mandelbrot(r: f64, i: f64) -> bool {
    let c = Complex::new(r, i);
    let mut z = Complex::new(0.0, 0.0);
    for _ in 0..ITERATIONS {
        z = z.clone() * z.clone() + c.clone();
        let val = z.clone().abs_sq();
        if val > 4.0 {
            return false;
        }
    }
    return true;
}

