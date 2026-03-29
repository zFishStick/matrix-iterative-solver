mod solvers;
mod input;

use matrix_iterative_solver::solvers::iterative_solver::IterativeSolver;
use matrix_iterative_solver::solvers::jacobi::JacobiSolver;
use nalgebra::{DVector, Matrix2, Matrix3, Matrix3x4};
use nalgebra_sparse::CsrMatrix;

// use crate::input::read_matrix_manual;

fn main() {

    let matrix_file : &str = "data/spa1.mtx";
    
    // tol = [10−4,10−6,10−8,10−10].
    let base = 10.0_f64;
    let exp = -4;
    let tol = base.powi(exp);
    println!("Tolerance: {}", tol);

    // let matrix : CsrMatrix<f64> = read_matrix_manual(matrix_file);
    let base_matrix = Matrix3::new(
        5.0, -2.0, 3.0,
        -3.0, 9.0, 1.0,
        2.0, -1.0, -7.0
    );
    
    let matrix : CsrMatrix<f64> = CsrMatrix::from(&base_matrix);

    // Extract the right-hand side vector b from the last column of the base matrix
    let b: DVector<f64> = DVector::from_vec(vec![-1.0, 2.0, 3.0]);

    let max_iter = 20000; 

    let jacobi_solver = JacobiSolver;
    
    let result = jacobi_solver.solve(&matrix, &b, tol, max_iter);

    println!("Jacobi Solver:");
    println!("Iterations: {}", result.iterations);
    println!("Solution: {:?}", result.solution);
    println!("Relative Error: {:.2e}", result.relative_error);
    println!("Converged: {}", result.converged);
    println!("Execution Time: {:.2?}", result.execution_time);

}
