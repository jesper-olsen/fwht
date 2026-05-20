use std::ops::{Add, Sub, Div};

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

//pub fn fwht<T>(x: &mut [T])
//where
//    T: Add<Output = T> + Sub<Output = T> + Copy,
//{
//    let n = x.len();
//    debug_assert!(n.is_power_of_two(), "Input length must be a power of two");
//    let mut h = 1;
//
//    while h < n {
//        for i in (0..n).step_by(h * 2) {
//            for j in i..i + h {
//                // SAFETY: `i` ranges from 0 to n-1 in steps of 2h.
//                // `j` ranges from i to i + h - 1.
//                // The maximum index accessed is `j + h`, which equals `i + 2h - 1`.
//                // Since `i + 2h` is bounded by `n`, `j + h` is strictly less than `n`.
//                unsafe {
//                    let a = *x.get_unchecked(j);
//                    let b = *x.get_unchecked(j + h);
//
//                    *x.get_unchecked_mut(j) = a + b;
//                    *x.get_unchecked_mut(j + h) = a - b;
//                }
//            }
//        }
//        h *= 2;
//    }
//}

// iterators - may avoids bounds checks
//pub fn fwht<T>(x: &mut [T])
//where
//    T: Add<Output = T> + Sub<Output = T> + Copy,
//{
//    let n = x.len();
//    assert!(n.is_power_of_two(), "Input length must be a power of two");
//    let mut h = 1;
//
//    while h < n {
//        for chunk in x.chunks_exact_mut(h * 2) {
//            let (left, right) = chunk.split_at_mut(h);
//            for j in 0..h {
//                let a = left[j];
//                let b = right[j];
//                left[j] = a + b;
//                right[j] = a - b;
//            }
//        }
//        h *= 2;
//    }
//}


pub fn ifwht<T>(x: &mut [T])
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: std::fmt::Debug,
{
    fwht(x);
    let n_val = T::try_from(x.len()).expect("Slice length exceeds type capacity");
    for val in x.iter_mut() {
        *val = *val / n_val;
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
