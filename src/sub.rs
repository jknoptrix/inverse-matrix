use crate::Matrix;

use std::ops::Sub;

impl<T> Sub for Matrix<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut result = Matrix {
            data: [[rhs.data[0][0]; 4]; 4],
        };

        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }

        result
    }
}
