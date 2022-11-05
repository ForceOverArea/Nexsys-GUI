use super::*;

#[derive(Clone)]
#[derive(Debug)]
pub struct Variable {
    pub value: f64,
    pub domain: Option<[f64; 2]>
}
impl Variable {
    /// Instantiates a new `Variable` struct with a specified value and domain
    pub fn new(value: f64, domain:Option<[f64; 2]>) -> Variable {
        Variable {
            value, 
            domain,
        }
    }

    /// Adds some quantity to `self.value` if it remains in the bounds of `self.domain` 
    pub fn step(&mut self, qty: f64) {
        match self.domain {
            Some(bounds) => {
                if bounds[0] < self.value + qty && self.value + qty < bounds[1] {
                    self.value += qty;
                }
            }
            None => {
                self.value += qty;
            }
        }
    }

    /// returns a new `Variable` struct with the same data aside from a new `self.value`
    pub fn edit_val(&self, new_val: f64) -> Variable {
        Variable {
            value: new_val,
            domain: self.domain
        }
    }

    /// returns `self.value`
    pub fn as_f64(&self) -> f64 {
        self.value
    }
}

/// Returns the derivative of a function at a point
pub fn d_dx(mut func: impl FnMut(f64) -> f64, x: f64) -> f64 {
    ( func(x + 0.0001) - func(x) ) / 0.0001
}

/// Returns a function that returns the error w.r.t. only one variable
pub fn partial_func<'a>(
    func: impl Fn(&HashMap<&str, Variable>) -> f64 + 'a,
    guess: &HashMap<&'a str, Variable>,
    target: &'a str
) -> impl FnMut(f64) -> f64 + 'a {

    let mut temp = guess.clone();
    let i = guess[target].clone();

    move |x:f64| -> f64 {
        temp.insert(target, i.edit_val(x));
        func(&temp)
    }
}

/// Returns the magnitude of a mathematical vector given as a `HashMap<&str, Variable>`
pub fn mag(vec: &HashMap<&str, f64>) -> f64 {
    let mgntd: f64 = vec
    .iter().map(
        |i| {
            i.1 * i.1
        }
    )
    .sum::<f64>().sqrt();
    mgntd
}

/// Scales a given mathematical vector scaled down to a specified magnitude.
pub fn scale<'a>(vc: &'a mut HashMap<&str, f64>, scale: f64) {
    let mgntd = mag(vc);
    for v in vc {
        *v.1 *= scale / mgntd;
    }
}