use super::*;
use linalg::{ NxN, mat_vec_mul };
use mvcalc::SolverResult;

fn invertible_matrix_2() -> NxN {
    let res = vec![ 
        vec![-1.0, 1.0], 
        vec![1.5, -1.0] 
    ];
    NxN::from_cols(2, res, None)
}

fn invertible_matrix_3() -> NxN {
    let res = vec![
        vec![ 1.0, 2.0, -1.0], 
        vec![ 2.0, 1.0,  2.0],
        vec![-1.0, 2.0,  1.0] 
    ];
    NxN::from_cols(3, res, None)
}

fn checkit(my_sys: Vec<&str>, soln: HashMap<&str, f64>) -> () {

    let guess = soln.clone().iter().map(
        |v| {
            (*v.0, Variable::new(1.0, None))
        }
    ).collect();

    let ans = mv_newton_raphson(my_sys, guess, 0.001, 10);

    match ans {
        SolverResult::Ok(t) => {
            for v in soln {
                assert_eq!(t[v.0].as_f64().round(), v.1);
            }            
        },
        SolverResult::Warn(t) => panic!("Valid w/ warning: {:#?}", t),
        SolverResult::Err => panic!("Error")
    }
}

#[test]
fn test_mat_vec_mul() {
    let my_matrix = NxN::identity(3);

    let my_vec = vec![ 2.0, 2.0, 2.0 ];

    assert_eq!(
        mat_vec_mul(my_matrix, my_vec.clone()),
        my_vec
    )
}

#[test]
fn test_nxn_row_add() {
    let mut my_matrix = invertible_matrix_2();
    my_matrix.add_to_row(1, &vec![1.0, 2.0]);
    let check = vec![
        vec![-1.0, 2.0],
        vec![1.5, 1.0]
    ];

    assert_eq!(my_matrix.to_vec(), check);

    let mut my_matrix = invertible_matrix_3();
    my_matrix.add_to_row(1, &vec![-2.0, -1.0, -2.0]);
    let check = vec![ 
        vec![ 1.0, 0.0, -1.0], 
        vec![ 2.0, 0.0,  2.0],
        vec![-1.0, 0.0,  1.0] 
    ];

    assert_eq!(my_matrix.to_vec(), check);
}

#[test]
fn test_nxn_row_scale() {
    let mut my_matrix = invertible_matrix_2();

    my_matrix.scale_row(1, 0.0);

    let check = vec![
        vec![-1.0, 0.0],
        vec![1.5, 0.0]
    ];

    assert_eq!(my_matrix.to_vec(), check);
}

#[test]
fn test_nxn_row_get() {
    let my_matrix = invertible_matrix_2();

    assert_eq!(my_matrix.get_row(0), vec![-1.0, 1.5]);
    assert_eq!(my_matrix.get_row(1), vec![1.0, -1.0])
}

#[test]
fn test_nxn_invert() {
    let mut my_matrix = invertible_matrix_2();
    my_matrix.invert().unwrap();

    assert_eq!(my_matrix.to_vec(), vec![ vec![2.0, 2.0], vec![3.0, 2.0] ]);

    let mut my_matrix = invertible_matrix_3();
    my_matrix.invert().unwrap();
    let check = vec![
        vec![3.0/16.0, 0.25, -5.0/16.0], 
        vec![0.25, 0.0, 0.25],
        vec![-5.0/16.0, 0.25, 3.0/16.0] 
    ];

    assert_eq!(my_matrix.to_vec(), check)
}

#[test]
fn test_jacobian() {
    let my_sys = vec![
        "x^2 + y",
        "y   - x"
    ];

    let guess = HashMap::from([
        ("x", Variable::new(1.0, None)),
        ("y", Variable::new(1.0, None))
    ]);

    let my_j = jacobian(&my_sys, &guess);

    let cols = stitch_hm(my_j.vars.clone().unwrap(), my_j.to_vec());

    assert_eq!(cols["x"][0].round(),  2.0);
    assert_eq!(cols["x"][1].round(), -1.0);
    assert_eq!(cols["y"][1].round(),  1.0);
    assert_eq!(cols["y"][0].round(),  1.0);
}

#[test]
fn test_solver() {
    let case_a = vec![
        "x^2 + y",
        "y -   x"
    ];
    let soln_a = HashMap::from([("x",0.0),("y",0.0)]);

    let case_b = vec![
        "2*x + 5*y + 2*z - -38",
        "3*x - 2*y + 4*z - 17",
        "-6*x + y - 7*z - -12"
    ];
    let soln_b = HashMap::from([("x",3.0),("y",-8.0),("z",-2.0)]);

    checkit(case_a, soln_a);

    checkit(case_b, soln_b);
}

#[test]
fn test_newton_raphson() {
    let equation = "x^2 + x - 1";
    let guess = ("x", Variable::new(1.0, None));
    
    let ans = newton_raphson(
        equation, 
        guess, 
        0.0001, 
        500,
        None
    ).unwrap();

    println!("Soln: {:?}", ans.1);
    assert_eq!(ans.1.as_f64().round(), 1.0);
}