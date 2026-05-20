# fwht

A fast, in-place Walsh-Hadamard Transform (FWHT) for Rust.

This crate provides a highly optimized, zero-allocation implementation of the Fast Walsh-Hadamard Transform. The transform maps a vector into a basis of orthogonal square-wave functions consisting of `+1` and `-1` patterns.

## Features

* **Zero allocations:** Operates entirely in-place.
* **Fast:** Runs in $O(N \log N)$ time. The core butterfly loop elides bounds checks to ensure optimal auto-vectorization.
* **Generic:** Works over any type implementing `Add`, `Sub`, and `Copy` (e.g., `i32`, `i64`, `f32`, `f64`).
* **Minimal:** `no_std` compatible by default.

## Usage

The input slice length must be a power of two.

```rust
use fwht::fwht;

fn main() {
    let mut x = [1i32, 1, -1, -1];
    
    // Compute the in-place transform
    fwht(&mut x);
    
    assert_eq!(x, [0, 0, 4, 0]);
}
```

## Examples
The repository includes examples demonstrating practical applications of the FWHT. You can run them using cargo run --example <name>.

1. Boolean Function Non-Linearity
Calculates the non-linearity of a Boolean function (XOR) by mapping its truth table to the Walsh domain and analyzing the maximum absolute value of its Walsh spectrum.

```Bash
cargo run --example boolean
```

2. Matrix Visualization
Generates the basis matrices for the Walsh-Hadamard Transform in both Natural (Sylvester) and Sequency (Walsh) ordering. Demonstrates how to map between the two using bit-reversal and Gray code decoding.

This example generates .dat files that can be rendered natively with gnuplot:

```bash
cargo run --example matrix_visualizer
gnuplot visualize.gp
```

## Algorithm Notes

The transform is computed iteratively using butterfly operations:a, b -> a + b, a - b
Because the Walsh-Hadamard Transform is symmetric, the inverse transform is identical to the forward transform, followed by scaling each element by $1/N$.
