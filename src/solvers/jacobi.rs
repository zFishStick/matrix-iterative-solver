use nalgebra::DVector;
use nalgebra_sparse::CsrMatrix;
use std::time::Instant;

use crate::solvers::iterative_solver::{IterativeSolver, SolverResult};


#[allow(dead_code)]
pub struct JacobiSolver;

impl IterativeSolver for JacobiSolver {
    fn solve(&self, matrix: &CsrMatrix<f64>, b: &DVector<f64>, tol: f64, max_iter: usize) -> SolverResult {
        let n = matrix.nrows();
        let mut x : DVector<f64> = DVector::from_vec(vec![1.0; n]);
        let b_norm = b.norm();
        let start_time = Instant::now();
        let mut iterations = 0;

        let mut _ax = DVector::zeros(n);
        let mut r = DVector::zeros(n);

        let diag: Vec<f64> = (0..n).map(|i| {
            let v = matrix.get_entry(i, i).expect("Missing diagonal element").into_value();
            if v == 0.0 { panic!("Error: zero diagonal element at row {}", i); }
            v
        }).collect();

        loop {
            _ax = matrix * &x; // This affect performance (FIX)
            r.copy_from(b);
            r -= &_ax;

            let res_norm = r.norm();
            let rel_res = res_norm / b_norm;

            if rel_res < tol || iterations >= max_iter {
                break;
            }

            for i in 0..n {
                x[i] += r[i] / diag[i];
                x[i] = (x[i] * 1000.0).round() / 1000.0; // This affect performance (FIX)
            }

            iterations += 1;

            if rel_res > 1e10 {
                println!("Warning: Divergence detected at iteration {}: relative residual = {:.2e}", iterations, rel_res);
                break;
            }
        }

        _ax = matrix * &x;
        r.copy_from(b);
        r -= &_ax;

        SolverResult {
            solution: x.as_slice().to_vec(),
            iterations,
            relative_error: r.norm() / b_norm,
            converged: iterations < max_iter,
            execution_time: start_time.elapsed(),
        }
    }
}
