use super::components::*;
use super::*;

#[test]
fn test_d_dx() {
    let derivative = d_dx(|x:f64| x * x, 2.0);
    assert!(3.999 <= derivative && derivative <= 4.001)
}

#[test]
fn test_mag() {
    let myvec = HashMap::from([
        ("x", 3.0),
        ("y", 4.0)
    ]);
    let check: f64 = 3.0 * 3.0 + 4.0 * 4.0;
    assert_eq!(mag(&myvec), check.sqrt())
}

#[test]
fn test_scale() {
    let mut myvec = HashMap::from([
        ("x", 3.0),
        ("y", 4.0)
    ]);
    let check = HashMap::from([
        ("x", 15.0),
        ("y", 20.0)
    ]);

    scale(&mut myvec, 25.0);

    assert_eq!(myvec, check)
}

#[test]
fn test_partial_func() {

    let system = "x + y = 9\nx - y = 4";

    let my_sys = |guess: &HashMap<&str, Variable>| -> f64 {
        system.split("\n").map(
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

    let my_guess: HashMap<&str, Variable> = HashMap::from([
        ("x", Variable::new(1.0, None)),
        ("y", Variable::new(1.0, None))
    ]);

    let mut pf_wrt_x = partial_func(my_sys, &my_guess, "x");
    let mut pf_wrt_y = partial_func(my_sys, &my_guess, "y");

    assert_eq!(pf_wrt_x(6.5), 3_f64);
    assert_eq!(pf_wrt_y(2.5), 11_f64);

}

#[test]
fn check_derivative() {
    let system = "x + y = 9\nx - y = 4";

    let my_sys = |guess: &HashMap<&str, Variable>| -> f64 {
        system.split("\n").map(
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

    let my_guess: HashMap<&str, Variable> = HashMap::from([
        ("x", Variable::new(1.0, None)),
        ("y", Variable::new(1.0, None))
    ]);

    let pf_wrt_x = partial_func(my_sys, &my_guess, "x");
    let pf_wrt_y = partial_func(my_sys, &my_guess, "y");

    assert_eq!(d_dx(pf_wrt_x, 1.0).round(), -2.0);
    assert_eq!(d_dx(pf_wrt_y, 1.0).round(), 0.0);

}

#[test]
fn test_solve() {

    let sys1 = "x + y = 9\nx - y = 4";
    let ans1 = solve(sys1, HashMap::new());

    assert!(
        6.49 < ans1["x"] && ans1["x"] < 6.51 &&
        2.49 < ans1["y"] && ans1["y"] < 2.51
    );

    let sys2 = "x^2 - y = 0\nz - x = 1\nx + y = 1";
    let ans2 = solve(sys2, HashMap::new());

    assert!(
        0.617 < ans2["x"] && ans2["x"] < 0.618 &&
        0.381 < ans2["y"] && ans2["y"] < 0.382 &&
        1.617 < ans2["z"] && ans2["z"] < 1.618
    );
}