use nalgebra_sparse::CsrMatrix;
use nalgebra::DVector;

#[allow(dead_code)]
pub struct SolverResult {
    pub solution: Vec<f64>,
    pub iterations: usize,
    pub relative_error: f64,
    pub converged: bool,
    pub execution_time: std::time::Duration,
}

#[allow(dead_code)]
pub trait IterativeSolver {
    fn solve(
        &self, 
        matrix: &CsrMatrix<f64>,
        b: &DVector<f64>,
        tol: f64,
        max_iter: usize
    ) -> SolverResult;
}