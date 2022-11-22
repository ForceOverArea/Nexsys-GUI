use super::{ functionify, split_hm, stitch_hm };
use super::linalg::{ NxN, mat_vec_mul };
use meval::{ eval_str_with_context, Context };
use std::collections::HashMap;

/// Effectively an `f64`, but with an optional domain that the value must be on.
#[derive(Clone)]
#[derive(Debug)]
pub struct Variable {
    value: f64,
    domain: Option<[f64; 2]>
}
impl Variable {
    /// Instantiates a new `Variable` struct with a specified value and domain.
    pub fn new(value: f64, domain:Option<[f64; 2]>) -> Variable {
        Variable {
            value, 
            domain,
        }
    }

    /// Allows the ability to mutate `self.value` if the new value is on `self.domain`.
    pub fn change(&mut self, qty: f64) {
        match self.domain {
            Some(bounds) => {
                if bounds[0] < qty && qty < bounds[1] {
                    self.value = qty;
                } else if bounds[0] > qty { // if qty is o.o.b., then move self.value to the bound 
                    self.value = bounds[0];
                } else {
                    self.value = bounds[1];
                }
            }
            None => {
                // This comment is here exclusively to commemorate the STUPIDEST bug I have ever written:
                // self.value += qty; <- note how the variable's value is increased instead of changed
                //            ~~         if no domain is specified. 
                self.value = qty;
            }
        }
    }

    /// Allows the ability to mutate `self.value` by adding `qty` to it if the sum of `self.value` and `qty` is on `self.domain`.
    pub fn step(&mut self, qty: f64) {
        match self.domain {
            Some(bounds) => {
                if bounds[0] < self.value + qty && self.value + qty < bounds[1] {
                    self.value += qty;
                } else if bounds[0] > self.value + qty { // if qty is o.o.b., then move self.value to the bound 
                    self.value = bounds[0];
                } else {
                    self.value = bounds[1];
                }
            }
            None => {
                self.value += qty; // IT'S. THIS. LINE. EVERY. GODDAMN. TIME.
            }
        }
    }

    /// Returns `self.value` as `f64`.
    pub fn as_f64(&self) -> f64 {
        self.value
    }
}

/// A result from the Newton-Raphson Solver that can be valid, valid with warnings, or erroneous.
pub enum SolverResult<T> {
    Ok(T),
    Warn(T),
    Err
}
impl <'a, T>SolverResult<T> {
    /// Returns the contained value, consuming the `self` value in the process. If the enum is a `SolverResult::Err`, this method panics.
    pub fn unwrap(self) -> T {
        match self {
            SolverResult::Ok(t) => t,
            SolverResult::Warn(t) => t,
            SolverResult::Err => panic!()
        }
    }
}

/// Returns the derivative of a function at a point.
pub fn d_dx(mut func: impl FnMut(f64) -> f64, x: f64) -> f64 {
    let dx = 1e-7;
    ( func(x + dx) - func(x) ) / dx
}

/// Solves a single equation for a single unknown value. 
/// `mv_newton_raphson` can also be used for this scenario, but this 
/// function is more lightweight and reasonable choice.
/// # Example
/// ```
/// use nexsys_core::mvcalc::*;
/// 
/// let my_eqn = "x^2 - 1";
/// let my_guess = ("x", Variable::new(-1.0, Some([-10.0, 0.0])));
/// 
/// let root = newton_raphson(my_eqn, my_guess, 0.001, 500, None).unwrap();
/// 
/// assert_eq!(root.1.as_f64(), -1.0)
/// ```
pub fn newton_raphson<'a>(
    equation: &'a str, 
    guess: (&'a str, Variable),
    tolerance: f64,
    max_iterations: i32,
    params: Option<HashMap<&str, Variable>>
) -> SolverResult<(&'a str, Variable)> {

    let mut xi = guess.1;
    let mut ctx = Context::new();

    if let Some(p) = params {
        for v in p {
            ctx.var(v.0, v.1.as_f64());
        }
    }

    // Lord, forgive me for what I am about to do...
    let mut f = |x:f64| eval_str_with_context(equation, ctx.var(guess.0, x))
        .expect("ERR: Failed to evaluate equation!").abs();

    let mut count = 0;
    while &f(xi.as_f64()) > &tolerance {

        let mut roc = d_dx(&mut f, xi.as_f64());
        if roc == 0.0 { roc = f64::MIN_POSITIVE } // Avoid crash
        xi.step( -&f(xi.as_f64()) / roc );
        
        count += 1;
        if count > max_iterations {
            return SolverResult::Warn((guess.0, xi))
        }
    }
    SolverResult::Ok((guess.0, xi))
}

/// Returns the partial derivative of a function w.r.t. the `target` variable.
/// # Example
/// ```
/// use nexsys_core::mvcalc::partial_d_dx;
/// use nexsys_core::mvcalc::Variable;
/// use std::collections::HashMap;
/// let expr = "x^2 + y - z";
/// 
/// let X = HashMap::from([
///     ("x", Variable::new(1_f64, None)),
///     ("y", Variable::new(1_f64, None)),
///     ("z", Variable::new(1_f64, None))
/// ]);
/// 
/// let dFdx = partial_d_dx(expr, &X, "x");
/// assert_eq!(dFdx.round(), 2_f64);
/// ```
pub fn partial_d_dx<'a>(
    expr: &str, 
    guess: &HashMap<&'a str, Variable>, 
    target: &'a str 
) -> f64 {

    // copy the guess vector
    let mut temp = guess.clone();

    // create an actual function from the given expression
    let func = functionify(expr);

    // create a partial function of the target variable
    let partial = move |x:f64| -> f64 {
        if let Some(v) = temp.get_mut(target) {
            v.change(x);
        }
        func(&temp)
    };

    // take the derivative of the partial function
    d_dx(partial, guess[target].as_f64())
}

/// Returns the (numerical) `NxN` Jacobian matrix of a given system of equations at the vector given by `guess`.
/// 
/// Note that the resulting matrix's columns will be in a random order, so extra care is needed to identify which
/// variable occupies which column by checking the ordering of `self.vars`.
/// # Example
/// ```
/// use nexsys_core::mvcalc::{ jacobian, Variable };
/// use std::collections::HashMap;
/// 
/// let my_sys = vec![
///     "x^2 + y",
///     "y   - x"
/// ];
/// let guess = HashMap::from([
///     ("x", Variable::new(1.0, None)),
///     ("y", Variable::new(1.0, None))
/// ]);
/// 
/// let j = jacobian(&my_sys, &guess);
/// 
/// // j.to_vec() will return roughly:
/// // vec![
/// //      vec![2.0, -1.0],
/// //      vec![1.0, 1.0]
/// // ];
/// ```
pub fn jacobian<'a>(system: &Vec<&str>, guess: &HashMap<&str, Variable>) -> NxN {
    if system.len() != guess.keys().len() { 
        panic!("ERR: System is not properly constrained!") // guard clause against invalid problems
    } 

    let size = system.len();
    let mut mat = Vec::new();
    let vec = split_hm(guess.clone());

    for c in 0..size {
        let col = Vec::from_iter(
            system.iter().map(
                |i| partial_d_dx(i, guess, vec.0[c])
            )
        );
        mat.push(col);
    };

    NxN::from_cols( size, mat, Some(vec.0) )
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
/// use nexsys_core::mvcalc::*;
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