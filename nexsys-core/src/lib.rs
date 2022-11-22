/// A module for linear algebra-related code. Contains the `NxN` 
/// struct and a few functions for matrix/vector math. 
pub mod linalg;

/// A module for multivariate calculus-related code. Contains functions
/// relevant to finding (approximate) derivatives and jacobian matrices, 
/// as well as single-variable and multivariate rust implementations of 
/// Newton's Method.
pub mod mvcalc;

#[cfg(test)]
mod tests;

/// This module exposes parts of this crate in a Python-compatible fashion. While
/// not very useful for Rust development, this allows the crate to be `pip-installed` as a 
/// Python package for use elsewhere.
/// 
/// Documentation on how these work Python-wise can be found on the 
/// PyO3 user guide here: https://pyo3.rs/v0.8.1/print.html
#[cfg(feature = "python_ffi")]
pub mod pyffi;

use mvcalc::*;
use meval::{ Context, eval_str_with_context };
use std::{ collections::HashMap };

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