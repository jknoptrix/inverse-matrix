mod determinant;
mod add;
mod neg;
mod sub;
mod mul;
mod div;

use crate::determinant::determinant;

#[derive(Debug)]
pub struct Matrix<T> {
    data: [[T; 4]; 4],
}

pub trait Invertible {
    fn invert(&self) -> Option<Self> where Self: Sized;
}

impl Invertible for Matrix<f32> {
    fn invert(&self) -> Option<Self> {
        // Calculate the determinant of the matrix
        let det = determinant(self);
        // If the determinant is zero, the matrix is not invertible
        if det == 0.0 {
            return None;
        }

        // Create a copy of the matrix data
        let mut a = self.data.clone();
        // Initialize b as an identity matrix
        let mut b = [[0.0; 4]; 4];
        for i in 0..4 {
            b[i][i] = 1.0;
        }

        // Forward pass of Gaussian elimination
        for i in 0..3 {
            for j in (i + 1)..4 {
                let factor = a[j][i] / a[i][i];
                for k in i..4 {
                    a[j][k] -= factor * a[i][k];
                }
                for k in 0..4 {
                    b[j][k] -= factor * b[i][k];
                }
            }
        }

        // Backward pass of Gaussian elimination
        for i in (0..4).rev() {
            for j in (i + 1..4).rev() {
                let factor = a[j][i] / a[i][i];
                for k in 0..4 {
                    b[j][k] -= factor * b[i][k];
                }
            }
            let factor = 1.0 / a[i][i];
            for k in 0..4 {
                b[i][k] *= factor;
            }
        }

        // Return the inverse matrix
        Some(Matrix { data: b })
    }
}

pub fn invert_matrix(matrix: Matrix<f32>) -> Option<Matrix<f32>> {
    matrix.invert()
}

fn main() {
    let a = Matrix {
        data: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 3.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };
    let inverse = invert_matrix(a);
    println!("{}", inverse.unwrap());
}