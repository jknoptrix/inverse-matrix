use crate::Matrix;

use std::ops::Div;

impl<T> Div<T> for Matrix<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, scalar: T) -> Self {
        let mut result = Matrix {
            data: [[self.data[0][0]; 4]; 4],
        };

        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][j] / scalar;
            }
        }

        result
    }
}