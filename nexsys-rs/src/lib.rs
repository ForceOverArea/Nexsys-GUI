#[cfg(test)]
mod test;
mod components;

use components::*;
use meval::{Context, eval_str_with_context};
use pyo3::{
    pyfunction, 
    pymodule, 
    PyResult,
    Python,
    types::{PyModule}, 
    wrap_pyfunction
};
use regex::Regex;
use std::collections::HashMap;

/// Solves a system of equations given as a `&str`. This is effectively a wrapper 
/// for the rest of the code in the solver engine.
pub fn solve<'a>(system: &'a str, bounds: HashMap<&str, [f64; 2]>) -> HashMap<&'a str, f64> {
    
    // Identify variables
    let re = Regex::new(r#"(?i)[a-z_]+"#).unwrap();
    let mut variables: HashMap<&str, Variable> = re.find_iter(system).map(
        |i| {
            let mut bds = None;
            if bounds.contains_key(i.as_str()) {
                bds = Some(bounds[i.as_str()])
            }
            (i.as_str(), Variable::new(1.0, bds))
        }
    ).collect();

    // Establish correctness function
    let correctness = |guess: &HashMap<&str, Variable>| -> f64 {
        let error: f64 = system.split("\n").map(
            |i| {
                let mut ctx = Context::new();
                
                for j in guess {
                    ctx.var(*j.0, j.1.as_f64()); 
                }
                
                let exp = i.replace("=", "-");
                let error_msg = format!("Correctness function failed to evaluate the system string: {}", &exp);
                
                eval_str_with_context(&exp, ctx)
                .expect(&error_msg)
                .abs()
            }
        ).sum();
        1.0 / error
    };

    // Establish threshold for when an answer is "correct" 
    // and a starting minimum step size.
    let limit = 1e7; 
    let mut step_size = 1.0;

    while correctness(&variables) < limit {

        let starting_correctness = correctness(&variables);
        //println!("correctness: {}\nguess:{:#?}", starting_correctness, variables);
        let mut change_vector: HashMap<&str, f64> = HashMap::new();

        for v in &variables {
            change_vector.insert( 
                *v.0,
                d_dx(
                    partial_func(correctness, &variables, v.0),
                    v.1.as_f64()
                )
            );
        }
        //println!("gradient: {:#?}", change_vector);

        if mag(&change_vector) > step_size {
            scale(&mut change_vector, step_size);
        } 

        let record = variables.clone();
        for v in change_vector {
            if let Some(j) = variables.get_mut(v.0) {
                j.step(v.1)
            }
        }

        if starting_correctness > correctness(&variables) {
            variables = record;
            if step_size > 0.00001 {
                step_size *= 0.1;
            } else {
                break;
            }
        }
    }

    variables.iter().map(
        |i| {
            (*i.0, i.1.as_f64())
        }
    ).collect()
}

// Code below this line is for use by the Python wrapper

#[pyfunction]
fn py_solve<'a>(system: &'a str, bounds: HashMap<&str, Vec<f64>> ) -> PyResult<HashMap<&'a str, f64>> {
    let bounds: HashMap<&str, [f64; 2]> = bounds.iter().map(
        |i| (*i.0, [i.1[0], i.1[1]])
    ).collect();
    let res = solve(system, bounds);
    Ok(res)
}

#[pymodule]
fn nexsys_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_solve, m)?)?;
    Ok(())
}