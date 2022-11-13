/// Returns the dot product of two given vectors.
pub fn vec_vec_dot(lhs: &Vec<f64>, rhs: &Vec<f64>) -> f64 {
    if lhs.len() != rhs.len() {
        // guard clause against illegal operation
        panic!("ERR: Vectors are different sizes!");
    }
    let mut count = 0;
    lhs.iter().map(
        |i| {
            let res = i * rhs[count];
            count += 1;
            res
        }
    ).sum()
}

/// Multiplies a matrix and a column vector.
pub fn mat_vec_mul(lhs: NxN, rhs: Vec<f64>) -> Vec<f64> {
    if lhs.size != rhs.len() {
        // guard clause against illegal operation
        panic!("ERR: Vectors are different sizes!");
    }

    let mat = lhs.to_vec();
    let mut res = Vec::new();

    for i in 0..rhs.len() {

        let mut row = Vec::new();

        for j in 0..rhs.len() {
            row.push(mat[j][i]);
        }

        res.push(vec_vec_dot(&row, &rhs));
    
    }
    res
}

/// Scales a vector by the given value.
pub fn scale_vec(vec: Vec<f64>, scalar: f64) -> Vec<f64> {
    vec.iter().map(
        |i| {
            i * scalar
        }).collect()
}

/// An n x n matrix with a given `size` n and a `Vec` containing the variables in each column if applicable.
#[derive(Debug)]
pub struct NxN {
    pub size: usize,
    mat: Vec<Vec<f64>>,
    pub vars: Option<Vec<String>>
}
impl NxN {
    /// Initializes an NxN identity matrix of the specified size
    /// # Example
    /// ```
    /// use nexsys_core::linalg::NxN;
    /// 
    /// let my_matrix = NxN::identity(3);
    /// let check = vec![ 
    ///     vec![1.0, 0.0, 0.0], 
    ///     vec![0.0, 1.0, 0.0], 
    ///     vec![0.0, 0.0, 1.0] 
    /// ];
    /// 
    /// assert_eq!(my_matrix.to_vec(), check);
    /// ```
    pub fn identity(size: usize) -> NxN {
        let mut mat = Vec::new();
        for i in 0..size {
            let mut col = Vec::new();
            for j in 0..size {
                if i == j {
                    col.push(1_f64);
                } else {
                    col.push(0_f64);
                }
            }
            mat.push(col);
        }
        NxN { size, mat, vars: None }
    }

    /// Initializes an NxN matrix of given values from a `Vec<Vec<f64>>`
    /// # Example
    /// ```
    /// use nexsys_core::linalg::NxN;
    /// 
    /// let my_cols = vec![
    ///     vec![1.0, 2.0, 3.0],
    ///     vec![1.0, 2.0, 3.0],
    ///     vec![1.0, 2.0, 3.0]
    /// ];
    ///  
    /// let my_matrix = NxN::from_cols(3, my_cols.clone(), None);
    /// 
    /// assert_eq!(my_matrix.to_vec(), my_cols);
    /// ```
    pub fn from_cols(size: usize, cols: Vec<Vec<f64>>, vars: Option<Vec<&str>>) -> NxN {
        match vars {
            Some(v) => {
                let vars: Vec<String> = v.iter()
                    .map(|i| i.to_string() 
                ).collect();
                NxN { size, mat: cols, vars: Some(vars) }
            }
            None => {
                NxN { size, mat: cols, vars: None }
            }
        }
    }

    /// Mutates a row, scaling it by the given value
    /// # Example
    /// ```
    /// use nexsys_core::linalg::NxN;
    /// 
    /// let mut my_matrix = NxN::identity(3);
    /// let check = vec![ 
    ///     vec![1.0, 0.0, 0.0], 
    ///     vec![0.0, 2.0, 0.0], 
    ///     vec![0.0, 0.0, 1.0] 
    /// ];
    /// my_matrix.scale_row(1, 2.0);
    /// assert_eq!(my_matrix.to_vec(), check);
    /// ```
    pub fn scale_row(&mut self, row: usize, scalar: f64) { 
        let n = self.size;
        for i in 0..n {
            self.mat[i][row] *= scalar;
        }
    }

    /// Adds a given row vector to a row in the matrix
    /// # Example
    /// ```
    /// use nexsys_core::linalg::NxN;
    /// 
    /// let mut my_matrix = NxN::identity(3);
    /// let check = vec![ 
    ///     vec![1.0, 2.0, 0.0], 
    ///     vec![0.0, 3.0, 0.0], 
    ///     vec![0.0, 2.0, 1.0] 
    /// ];
    /// my_matrix.add_to_row(1, &vec![2.0, 2.0, 2.0]);
    /// assert_eq!(my_matrix.to_vec(), check);
    /// ```
    pub fn add_to_row(&mut self, row: usize, vec: &Vec<f64>) {
        let n = self.size;
        for i in 0..n {
            self.mat[i][row] += vec[i];
        }
    }

    /// Returns a row from the matrix
    /// # Example
    /// ```
    /// use nexsys_core::linalg::NxN;
    /// 
    /// let mut my_matrix = NxN::identity(3);
    /// 
    /// let check = vec![0.0, 0.0, 1.0];
    /// 
    /// assert_eq!(my_matrix.get_row(2), check);
    /// ```
    pub fn get_row(&self, row: usize) -> Vec<f64> {
        let n = self.size;
        let mut res = Vec::new();
        for i in 0..n {
            res.push(self.mat[i][row]);
        }
        res
    }

    /// inverts the matrix, if possible. This method returns a result that
    /// indicates whether the inversion was successful or not.
    /// # Example
    /// ```
    /// use nexsys_core::linalg::NxN;
    /// let res = vec![ 
    ///    vec![-1.0, 1.0], 
    ///    vec![1.5, -1.0] 
    /// ];
    /// let mut my_matrix = NxN::from_cols(2, res, None);
    /// my_matrix.invert().unwrap();
    /// 
    /// let inverse = vec![ 
    ///     vec![2.0, 2.0], 
    ///     vec![3.0, 2.0] 
    /// ];
    /// 
    /// assert_eq!(my_matrix.to_vec(), inverse);
    /// ```
    pub fn invert(&mut self) -> Result<(), ()> {
        let n = self.size;
        let mut inv = NxN::identity(n);

        for c in 0..n {
            for r in 0..n {
                if c == r {
                    continue; // guard clause against modifying the diagonal
                } else {
                    if self.mat[c][c] == 0.0 { 
                        return Err(())
                    }
                    // get the scalar that needs to be applied to the row vector
                    let scalar = - self.mat[c][r] / self.mat[c][c];

                    // create the row vector to add to self & row vector to add to inv
                    let v = scale_vec(self.get_row(c), scalar);
                    let vi = scale_vec(inv.get_row(c), scalar);

                    // add the vector to self
                    self.add_to_row(r, &v);

                    // perform the same operation on the identity matrix
                    inv.add_to_row(r, &vi);
                }
            }
        }

        for i in 0..n {
            let scalar = 1.0 / self.mat[i][i];
            self.scale_row(i, scalar);
            inv.scale_row(i, scalar);
        }

        // println!("{:?}", self.mat);

        // Assign the identity matrix's values to self.mat
        self.mat = inv.to_vec();
        Ok(())
    }

    /// Returns the matrix as `Vec<Vec<f64>>`, consuming the `self` value in the process
    pub fn to_vec(self) -> Vec<Vec<f64>> {
        self.mat
    }
}