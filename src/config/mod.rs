mod add;
mod div;
mod mul;
mod neg; // nig...
mod sub;

use crate::Matrix;
use std::fmt::{Display, Formatter, Result};

pub fn determinant(matrix: &Matrix<f32>) -> f32 {
    let mut m = matrix.data.clone();

    for i in 0..3 {
        for j in (i + 1)..4 {
            let factor = m[j][i] / m[i][i];
            for k in i..4 {
                m[j][k] -= factor * m[i][k];
            }
        }
    }

    m[0][0] * m[1][1] * m[2][2] * m[3][3]
}
// dont be scared but previous ver of this is:
/*
    m[0][0] * m[1][1] * m[2][2] * m[3][3]
        + m[0][0] * m[1][2] * m[2][3] * m[3][1]
        + m[0][0] * m[1][3] * m[2][1] * m[3][2]
        + m[0][1] * m[1][0] * m[2][3] * m[3][2]
        + m[0][1] * m[1][2] * m[2][0] * m[3][3]
        + m[0][1] * m[1][3] * m[2][2] * m[3][0]
        + m[0][2] * m[1][0] * m[2][1] * m[3][3]
        + m[0][2] * m[1][1] * m[2][3] * m[3][0]
        + m[0][2] * m[1][3] * m[2][0] * m[3][1]
        + m[0][3] * m[1][0] * m[2][2] * m[3][1]
        + m[0][3] * m[1][1] * m[2][0] * m[3][2]
        + m[0][3] * m[1][2] * m[2][1] * m[3][0]
        - m[0][0] * m[1][1] * m[2][3] * m[3][2]
        - m[0][0] * m[1][2] * m[2][1] * m[3][3]
        - m[0][0] * m[1][3] * m[2][2] * m[3][1]
        - m[0][1] * m[1][0] * m[2][2] * m[3][3]
        - m[0][1] * m[1][2] * m[2][3] * m[3][0]
        - m[0][1] * m[1][3] * m[2][0] * m[3][2]
        - m[0][2] * m[1][0] * m[2][3] * m[3][1]
        - m[0][2] * m[1][1] * m[2][0] * m[3][3]
        - m[0][2] * m[1][3] * m[2][1] * m[3][0]
        - m[0][3] * m[1][0] * m[2][1] * m[3][2]
        - m[0][3] * m[1][1] * m[2][2] * m[3][0]
        - m[0][3] * m[1][2] * m[2][0] * m[3][1]
 */

impl<T> Display for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "{} {} {} {}", self.data[0][0], self.data[0][1], self.data[0][2], self.data[0][3])?;
        writeln!(f, "{} {} {} {}", self.data[1][0], self.data[1][1], self.data[1][2], self.data[1][3])?;
        writeln!(f, "{} {} {} {}", self.data[2][0], self.data[2][1], self.data[2][2], self.data[2][3])?;
        write!(f, "{} {} {} {}", self.data[3][0], self.data[3][1], self.data[3][2], self.data[3][3])
    }
}