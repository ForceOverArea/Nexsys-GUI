use std::collections::HashMap;

use crate::{
    mv_newton_raphson, 
    mvcalc::Variable, 
    SolverResult
};

use pyo3::{
    pyfunction,
    pymodule,
    wrap_pyfunction,
    Python,
    PyResult,
    types::PyModule, exceptions::PyException
};

/// Provides Python access to the nexsys-core Multivariate Newton-Raphson equation solver via PyO3.
/// 
/// Note that this function is not tested in this crate as it is not meant for use in Rust. Instead, 
/// the code *wrapped in this function* has tests written for it that are used for validation. 
#[pyfunction]
fn py_mvnr<'a>(
    system: Vec<&str>, 
    guess: HashMap<&'a str, f64>,
    bounds: HashMap<&str, Vec<f64>>,
    tolerance: f64,
    max_iterations: i32
) -> PyResult<HashMap<&'a str, f64>> {
    
    let rustified_guess = guess.iter().map(
        |v| {
            let mut domain = None;
            if bounds.contains_key(*v.0) {
                domain = Some([bounds[*v.0][0], bounds[*v.0][1]]);
            }
            (*v.0 , Variable::new(*v.1, domain))
        }
    ).collect();
    
    let res = mv_newton_raphson(
        system, 
        rustified_guess, 
        tolerance, 
        max_iterations
    );

    match res {
        SolverResult::Ok(r) => {
            Ok(
                r.iter().map(
                    |v| {
                        (*v.0, v.1.as_f64())
                    }
                ).collect()
            )
        }
        SolverResult::Warn(r) => {
            return Ok(
                r.iter().map(
                    |v| {
                        (*v.0, v.1.as_f64())
                    }
                ).collect()
            )
        }
        SolverResult::Err => {
            return Err(
                PyException::new_err(
                    "solver encountered an error during iteration"
                )
            )
        }
    }
}

#[pymodule]
fn nexsys_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_mvnr, m)?)?;
    Ok(())
}