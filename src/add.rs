use crate::Matrix;

use std::ops::Add;

impl<T> Add for Matrix<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = Matrix {
            data: [[rhs.data[0][0]; 4]; 4],
        };

        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }

        result
    }
}