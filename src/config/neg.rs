use crate::Matrix;

use std::ops::Neg;

impl<T> Neg for Matrix<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(self) -> Self {
        let mut result = Matrix {
            data: [[self.data[0][0].neg(); 4]; 4],
        };

        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][j].neg();
            }
        }

        result
    }
}