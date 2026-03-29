use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use nalgebra_sparse::{CooMatrix, CsrMatrix};

#[allow(unused_assignments)]
pub fn read_matrix_manual(path: &str) -> CsrMatrix<f64> {
    let file = File::open(path).expect("Impossibile aprire il file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut rows = 0;
    let mut cols = 0;

    (rows, cols) = extract_matrix_info(&mut lines);

    let mut coo = CooMatrix::new(rows, cols);

    while let Some(line) = lines.next() {
        let l = line.expect("Reading line failed");
        let parts: Vec<&str> = l.split_whitespace().collect();
        
        if parts.len() == 3 {
            let r = parts[0].parse::<usize>().expect("Invalid row") - 1;
            let c = parts[1].parse::<usize>().expect("Invalid column") - 1;
            let v = parts[2].parse::<f64>().expect("Invalid value");

            coo.push(r, c, v);

            if r != c {
                coo.push(c, r, v);
            }
        }
    }

    CsrMatrix::from(&coo)
}

fn extract_matrix_info(lines: &mut Lines<BufReader<File>>) -> (usize, usize) {
    let mut dims : Vec<usize> = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        if line.starts_with('%') { continue; }
        
        dims = line
            .split_whitespace()
            .map(|s| s.parse().expect("Parsing dimension failed"))
            .collect();
        
        if dims.len() >= 2 {
            println!("Matrix dimensions: {}x{}, non-zero entries: {}", dims[0], dims[1], dims[2]);
            break;
        }
    }
    (dims[0], dims[1])
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_matrix() {
        let matrix = read_matrix_manual("data/spa1.mtx");
        println!("Matrix:\n{:?}", matrix);
    }
}