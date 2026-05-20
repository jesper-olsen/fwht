// examples/matrices.rs
use std::fs::File;
use std::io::{BufWriter, Write};
use fwht::fwht; // Assuming this is your crate

/// Converts a Natural index to a Sequency index (number of zero-crossings)
fn natural_to_sequency(natural_index: usize, k: u32) -> usize {
    // 1. Bit-reverse the index within the k-bit field
    let reversed = natural_index.reverse_bits() >> (usize::BITS - k);
    
    // 2. Gray-decode the reversed bits
    let mut gray = reversed;
    gray ^= gray >> 16;
    gray ^= gray >> 8;
    gray ^= gray >> 4;
    gray ^= gray >> 2;
    gray ^= gray >> 1;
    
    gray
}

fn main() -> std::io::Result<()> {
    let k = 6; // 2^6 = 64x64 matrix
    let n = 1 << k;

    let mut natural_matrix = vec![vec![0; n]; n];
    let mut sequency_matrix = vec![vec![0; n]; n];

    // Generate the Natural matrix
    for i in 0..n {
        let mut row = vec![0; n];
        row[i] = 1; 
        fwht(&mut row); // FWHT of a delta pulse yields the basis row
        
        // Map to Sequency ordering
        let seq_idx = natural_to_sequency(i, k);
        
        natural_matrix[i] = row.clone();
        sequency_matrix[seq_idx] = row;
    }

    // Write matrices to .dat files for gnuplot
    write_matrix("natural.dat", &natural_matrix)?;
    write_matrix("sequency.dat", &sequency_matrix)?;

    println!("Generated natural.dat and sequency.dat.");
    println!("Plot them with\n   gnuplot scripts/matrices.gp");
    Ok(())
}

fn write_matrix(filename: &str, matrix: &[Vec<i32>]) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    for row in matrix {
        for (i, val) in row.iter().enumerate() {
            if i > 0 { write!(writer, " ")?; }
            write!(writer, "{val}")?;
        }
        writeln!(writer)?;
    }
    Ok(())
}
