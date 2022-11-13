pub mod linalg;
pub mod mvcalc;
#[cfg(test)]
mod tests;

#[cfg(feature = "python_ffi")]
pub mod pyffi;

use linalg::*;
use mvcalc::*;
use meval::{ Context, eval_str_with_context };
use std::collections::HashMap;

/// A result from the Newton-Raphson Solver that can be valid, valid with warnings, or erroneous.
pub enum SolverResult<T> {
    Ok(T),
    Warn(T),
    Err
}
impl <'a>SolverResult<HashMap<&'a str, Variable>> {
    /// Returns the contained value, consuming the `self` value in the process. If the enum is a `SolverResult::Err`, this method panics.
    pub fn unwrap(self) -> HashMap<&'a str, Variable> {
        match self {
            SolverResult::Ok(t) => t,
            SolverResult::Warn(t) => t,
            SolverResult::Err => panic!()
        }
    }
}

/// Returns a `HashMap` indicating where a given HashMap's keys will be placed in a Vec.
#[allow(dead_code)]
fn index_map<'a, V>(hm: &HashMap<&'a str, V>) -> HashMap<&'a str, usize> {
    let mut i: usize = 0;
    let mut res = HashMap::new();
    for k in hm.keys() {
        res.insert(*k, i);
        i += 1;
    }
    res
}

/// Returns a tuple of `Vec`s that contain the keys and values of the original HashMap. 
/// The index of the key will be the same as its corresponding value's index.
fn split_hm<K, V>(hm: HashMap<K, V>) -> (Vec<K>, Vec<V>) {
    let mut keys = Vec::new();
    let mut vals = Vec::new();

    for i in hm {
        keys.push(i.0);
        vals.push(i.1);
    }

    (keys, vals)
}

/// Reverses the operation performed by `split_hm`.
fn stitch_hm<K: std::hash::Hash + std::cmp::Eq, V>(mut keys: Vec<K>, mut vals: Vec<V>) -> HashMap<K, V> {
    let mut res = HashMap::new();
    for _ in 0..keys.len() {
        res.insert(
            keys.pop().unwrap(), 
            vals.pop().unwrap()
        );
    }
    res
}

/// Takes a mathematical expression given as a `&str` and returns a function that takes a `HashMap<&str, Variable>` and returns an `f64`.
fn functionify<'a>(text: &'a str) -> impl Fn(&HashMap<&str, Variable>) -> f64 + 'a {
    let func = move |v:&HashMap<&str, Variable>| -> f64 {
        let mut ctx = Context::new();
        
        for k in v {
            ctx.var(*k.0, k.1.as_f64());
        }

        eval_str_with_context(text, ctx)
            .expect(&format!("ERR: Failed to evaluate expression: {}", text))
    };
    func
}

/// Performs one iteration of Newton's method for a system of equations, returning the next guess vector. 
fn newton_iteration<'a>(system: &Vec<&str>, mut guess: HashMap<&'a str, Variable>) -> Result<HashMap<&'a str, Variable>, ()> {
    let mut j = jacobian(system, &guess);
    // println!("JACOBIAN: \n{:#?}", &j);

    let inv_result = j.invert();
    if let Err(()) = inv_result {
        return Err(()) // Return an error if the matrix is non-invertible
    }

    let fx = Vec::from_iter(
        system.iter().map(
            |i| functionify(i)(&guess)
        )
    );
    let x_n = stitch_hm(
        j.vars.clone().unwrap(),
        mat_vec_mul(j, fx)
    );
    // println!("CHANGE: \n{:#?}", x_n);
    for v in &mut guess {
        v.1.step(-x_n[&v.0.to_string()])
    }

    // println!("GUESS VECTOR: \n{:#?}", guess);
    Ok(guess)
}

/// Attempts to solve the equations passed to `system` via the Newton-Raphson method.
/// # Example
/// ```
/// use std::collections::HashMap;
/// use nexsys_core::mv_newton_raphson;
/// use nexsys_core::mvcalc::Variable;
/// 
/// let my_sys = vec!["x^2 + y", "y - x"];
/// let guess = HashMap::from([
///     ("x", Variable::new(1.0, None)),
///     ("y", Variable::new(1.0, None))
/// ]);
/// let ans = mv_newton_raphson(my_sys, guess, 0.001, 500).unwrap();
/// 
/// assert_eq!(ans["x"].as_f64().round(), 0.0)
/// ```
pub fn mv_newton_raphson<'a>(
    system: Vec<&str>, 
    mut guess: HashMap<&'a str, Variable>,
    tolerance: f64,
    max_iterations: i32
) -> SolverResult<HashMap<&'a str, Variable>> {
    
    let error = |guess: &HashMap<&str, Variable>| -> f64 {
        system.iter().map(
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
        ).sum()
    };
    
    let mut count = 0;

    loop {
        // println!("ITERATION # {}", count);
        let res = newton_iteration(&system, guess);
        if let Err(()) = res {
            return SolverResult::Err;
        }
        guess = res.unwrap();

        let e = error(&guess);

        if e < tolerance { // Solution is valid and acceptable
            return SolverResult::Ok(guess)

        } else if count > max_iterations { // Solution is valid, but timed out
            guess.insert("__error__", Variable::new(e, None));
            return SolverResult::Warn(guess)

        } 
        count += 1;
    }
}