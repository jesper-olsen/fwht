// In cryptography, a Boolean function’s non-linearity measures its distance 
// from the set of all affine functions. We calculate this by converting the 
// truth table to a bipolar format (0->1, 1->-1) and running the FWHT. 
// The maximum absolute value in the resulting Walsh spectrum reveals how 
// linear the function is.

use fwht::fwht; 

fn main() {
    // Truth table for a 3-input XOR function: f(x, y, z) = x ^ y ^ z
    // Since XOR is perfectly linear, its non-linearity should be 0.
    let mut xor_function = [1i32, -1, -1, 1, -1, 1, 1, -1];
    
    println!("Original bipolar truth table: {:?}", xor_function);
    fwht(&mut xor_function);
    println!("Walsh Spectrum:               {:?}", xor_function);
    
    // Non-linearity formula: 2^(n-1) - 0.5 * max(|Walsh_coef|)
    let max_coef = xor_function.iter().map(|v| v.abs()).max().unwrap_or(0);
    let nonlinearity = 4 - (max_coef / 2);
    
    println!("Non-linearity: {}", nonlinearity);
    assert_eq!(nonlinearity, 0, "Pure XOR functions are perfectly linear");
}
