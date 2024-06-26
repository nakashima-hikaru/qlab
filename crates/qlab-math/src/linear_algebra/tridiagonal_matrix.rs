use crate::value::Value;
use std::ops::Mul;

#[derive(Debug)]
pub enum MatrixValidationError {
    MatrixShapeError,
}

pub(crate) struct TridiagonalMatrix<V: Value> {
    upper_diagonal: Vec<V>,
    diagonal: Vec<V>,
    lower_diagonal: Vec<V>,
    size: usize,
}

impl<V: Value> TridiagonalMatrix<V> {
    pub fn try_new(
        upper_diagonal: Vec<V>,
        diagonal: Vec<V>,
        lower_diagonal: Vec<V>,
    ) -> Result<Self, MatrixValidationError> {
        if !(upper_diagonal.len() == lower_diagonal.len()
            && upper_diagonal.len() + 1 == diagonal.len())
        {
            return Err(MatrixValidationError::MatrixShapeError);
        }
        Ok(Self {
            size: diagonal.len(),
            upper_diagonal,
            diagonal,
            lower_diagonal,
        })
    }

    // Solve Ax = b.
    pub fn solve(self, b: &[V]) -> Vec<V> {
        if self.size == 1 {
            return vec![b[0] / self.diagonal[0]];
        }
        // shape validation is already done at construction phase
        solve_with_thomas_algorithm_unchecked(
            self.size,
            self.lower_diagonal.as_slice(),
            self.diagonal.as_slice(),
            self.upper_diagonal.as_slice(),
            b,
        )
    }
}

impl<V: Value> Mul<Vec<V>> for TridiagonalMatrix<V> {
    type Output = Option<Vec<V>>;

    fn mul(self, rhs: Vec<V>) -> Self::Output {
        if rhs.len() != self.size {
            return None;
        }
        let mut ret = Vec::with_capacity(self.size);
        for i in 0..self.size {
            let mut temp = self.diagonal[i] * rhs[i];
            if i + 1 < self.size {
                temp += self.upper_diagonal[i] * rhs[i + 1];
            }
            if i > 0 {
                temp += self.lower_diagonal[i - 1] * rhs[i - 1];
            }

            ret.push(temp);
        }
        Some(ret)
    }
}

fn solve_with_thomas_algorithm_unchecked<V: Value>(
    matrix_size: usize,
    lower_diagonal: &[V],
    diagonal: &[V],
    upper_diagonal: &[V],
    b: &[V],
) -> Vec<V> {
    let mut x = b.to_vec();
    let mut scratch = Vec::with_capacity(matrix_size);
    scratch.push(upper_diagonal[0] / diagonal[0]);
    x[0] /= diagonal[0];

    /* loop from 1 to X - 1 inclusive */
    for ix in 1..matrix_size {
        if ix < matrix_size - 1 {
            scratch.push(
                upper_diagonal[ix] / (diagonal[ix] - lower_diagonal[ix - 1] * scratch[ix - 1]),
            );
        }
        x[ix] = (x[ix] - lower_diagonal[ix - 1] * x[ix - 1])
            / (diagonal[ix] - lower_diagonal[ix - 1] * scratch[ix - 1]);
    }

    /* loop from X - 2 to 0 inclusive */
    for ix in (0..matrix_size - 2).rev() {
        let temp = scratch[ix] * x[ix + 1];
        x[ix] -= temp;
    }
    x
}
