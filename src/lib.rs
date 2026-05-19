use std::ops::{Add, Sub};

/// Computes the in-place Fast Walsh-Hadamard Transform (FWHT).
///
/// The Walsh-Hadamard transform maps a vector into a basis of orthogonal
/// square-wave functions consisting only of `+1` and `-1` patterns.
///
/// This implementation:
///
/// - runs in `O(n log n)` time
/// - operates in-place
/// - allocates no additional memory
/// - uses only addition and subtraction
///
/// The input length must be a power of two.
///
/// # Algorithm
///
/// The transform is computed iteratively using butterfly operations:
///
/// ```text
/// a, b -> a + b, a - b
/// ```
///
/// At each stage, increasingly larger blocks are combined until the
/// full transform has been computed.
///
/// # Type Parameters
///
/// The element type `T` must support:
///
/// - addition
/// - subtraction
/// - copying
///
/// Typical choices are:
///
/// - `i32`
/// - `i64`
/// - `f32`
/// - `f64`
///
/// Bipolar hypervectors (`+-1`) naturally transform into integer-valued
/// Walsh-Hadamard coefficients.
///
/// # Example
///
/// ```
/// use fwht::fwht;
///
/// let mut x = [1i32, 1, -1, -1];
/// fwht(&mut x);
///
/// assert_eq!(x, [0, 0, 4, 0]);
/// ```
///
/// # Panics
///
/// Panics in debug builds if the input length is not a power of two.
///
pub fn fwht<T>(x: &mut [T])
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    let n = x.len();
    debug_assert!(n.is_power_of_two());
    let mut h = 1;

    while h < n {
        for i in (0..n).step_by(h * 2) {
            for j in i..i + h {
                let a = x[j];
                let b = x[j + h];

                x[j] = a + b;
                x[j + h] = a - b;
            }
        }

        h *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dim8() {
        const H8: [[i32; 8]; 8] = [
            [1, 1, 1, 1, 1, 1, 1, 1],
            [1, -1, 1, -1, 1, -1, 1, -1],
            [1, 1, -1, -1, 1, 1, -1, -1],
            [1, -1, -1, 1, 1, -1, -1, 1],
            [1, 1, 1, 1, -1, -1, -1, -1],
            [1, -1, 1, -1, -1, 1, -1, 1],
            [1, 1, -1, -1, -1, -1, 1, 1],
            [1, -1, -1, 1, -1, 1, 1, -1],
        ];

        let mut h8 = [[0; 8]; 8];
        for (row, h8_row) in h8.iter_mut().enumerate() {
            h8_row[row] = 1;
            fwht::<i32>(h8_row);
        }
        assert_eq!(&H8, &h8);
    }
}
