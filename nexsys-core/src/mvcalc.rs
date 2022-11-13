use super::{
    functionify, 
    split_hm, 
    linalg::NxN
};
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

/// Returns the derivative of a function at a point.
pub fn d_dx(mut func: impl FnMut(f64) -> f64, x: f64) -> f64 {
    let dx = 1e-7;
    ( func(x + dx) - func(x) ) / dx
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