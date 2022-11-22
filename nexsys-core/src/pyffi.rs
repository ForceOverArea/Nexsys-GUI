use std::collections::HashMap;

use super::{
    mv_newton_raphson, 
    mvcalc::{Variable, newton_raphson}, 
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

/// Provides Python access to the nexsys-core single-variable Newton-Raphson equation solver via PyO3.
/// 
/// Note that this function is not tested in this crate as it is not meant for use in Rust. Instead, 
/// the code *wrapped in this function* has tests written for it that are used for validation. 
#[pyfunction]
pub fn py_nr<'a>(
    equation: &'a str,
    guess: HashMap<&'a str, f64>,
    locals: HashMap<&str, f64>,
    bounds: HashMap<&str, Vec<f64>>,
    tolerance: f64,
    max_iterations: i32
) -> PyResult<HashMap<&'a str, f64>> {

    let rustified_guess = guess
        .iter()
        .map(|i| {

            let mut dmn = None;
            if bounds.contains_key(*i.0) {
                dmn = Some([bounds[*i.0][0], bounds[*i.0][1]]);
            }

            (*i.0, Variable::new(*i.1, dmn))

        }).collect::<Vec<(&str, Variable)>>()
        .pop()
        .unwrap();

    let params = Some(locals
        .iter()
        .map(|i| {

            let mut dmn = None;
            if bounds.contains_key(*i.0) {
                dmn = Some([bounds[*i.0][0], bounds[*i.0][1]]);
            }

            (*i.0, Variable::new(*i.1, dmn))

        }).collect::<HashMap<&str, Variable>>()
    );

    let res = newton_raphson(
        equation, 
        rustified_guess, 
        tolerance, 
        max_iterations, 
        params
    );

    match res {
        SolverResult::Ok(r) => {
            Ok(
                HashMap::from([(r.0, r.1.as_f64())])
            )
        }
        SolverResult::Warn(r) => {
            return Ok(
                HashMap::from([(r.0, r.1.as_f64())])
            )
        }
        SolverResult::Err => {
            return Err(
                PyException::new_err(
                "Solver encountered an error during iteration"
                )
            )
        }
    }
}


/// Provides Python access to the nexsys-core multivariate Newton-Raphson equation solver via PyO3.
/// 
/// Note that this function is not tested in this crate as it is not meant for use in Rust. Instead, 
/// the code *wrapped in this function* has tests written for it that are used for validation. 
#[pyfunction]
pub fn py_mvnr<'a>(
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
    m.add_function(wrap_pyfunction!(py_nr, m)?)?;
    Ok(())
}