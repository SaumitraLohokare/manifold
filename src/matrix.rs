use std::fmt::Display;

use rand::prelude::*;

use crate::{activations::Activation, errors::ManifoldError};

/// Structure that holds the data of a matrix. It implements some of the most common operations needed.
/// 
/// DT: Data type of the matrix elements
/// R: Number of rows
/// C: Number of columns
#[derive(Clone)]
pub struct Matrix {
    pub r: usize,
    pub c: usize,
    data: Vec<Vec<f32>>
}

impl Matrix {
    pub fn new(r: usize, c: usize) -> Self {
        let mut data = Vec::with_capacity(r);

        for _ in 0..r {
            data.push(vec![0.0; c]);
        }

        Self {
            r,
            c,
            data
        }
    }

    pub fn new_rand(r: usize, c: usize) -> Self {
        let mut rng = rand::thread_rng();
        
        let mut data = Vec::with_capacity(r);
        
        for _ in 0..r {
            data.push((0..c).map(|_| rng.gen::<f32>()).collect());
        }

        Self {
            r,
            c,
            data
        }
    }

    /// Adds two matrices to produce a new matrix. 
    /// 
    /// > Does not modify the original matrices.
    pub fn add(&self, other: &Self) -> Result<Self, ManifoldError> {
        if self.r != other.r || self.c != other.c {
            return Err(ManifoldError::IncompatibleMatrices);
        }

        let mut result = Matrix::new(self.r, self.c);
        for row in 0..self.r {
            for col in 0..self.c {
                result.data[row][col] = self.data[row][col] + other.data[row][col]
            }
        }

        Ok(result)
    }

    /// Multiplies two matrices to produce a new matrix. 
    /// 
    /// > Does not modify the original matrices.
    pub fn mul(&self, other: &Self) -> Result<Self, ManifoldError> {
        if self.c != other.r {
            return Err(ManifoldError::IncompatibleMatrices);
        }

        let mut result = Matrix::new(self.r, other.c);

        for row_idx in 0..self.r {
            for col_idx in 0..other.c {
                let mut sum = 0.0;
                for idx in 0..self.c {
                    sum += self.data[row_idx][idx] * other.data[idx][col_idx];
                }
                result.data[row_idx][col_idx] = sum;
            }
        }
        
        Ok(result)
    }

    pub fn apply_activation(&mut self, activation: Activation) {
        self.data.iter_mut().for_each(|x| activation.apply(x));
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        for row in 0..self.r {
            for col in 0..self.c {
                write!(f, "{:.4} ", self.data[row][col])?;
            }
            writeln!(f)?;
        }
        write!(f, "]")?;

        Ok(())
    }
}

impl<const R: usize, const C: usize> From<[[f32; C]; R]> for Matrix {
    fn from(value: [[f32; C]; R]) -> Self {
        let mut data = Vec::with_capacity(R);

        for row in value {
            data.push(row.to_vec());
        }

        Self {
            r: R,
            c: C,
            data
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn test_matrix_from_2d_array() {
        let x = [[0.0; 2]; 2];
        let control = vec![vec![0.0; 2]; 2];

        let result = Matrix::from(x);

        assert_eq!(result.data, control);
    }
}