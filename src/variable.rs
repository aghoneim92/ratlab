use std::{
    fmt::{Display, Formatter},
    ops::{Add, Div, Mul, Sub},
};

use nalgebra::DMatrix;

#[derive(Debug, Clone)]
pub enum Variable {
    Number(f64),
    Matrix(DMatrix<f64>),
}

impl Variable {
    pub fn pointwise_mul(&self, rhs: Variable) -> Result<Variable, String> {
        match (self, rhs) {
            (Variable::Number(_), Variable::Number(_)) => {
                Err("Cannot pointwise multiply two numbers".to_string())
            }
            (Variable::Number(_), Variable::Matrix(_)) => {
                Err("Cannot pointwise multiply a number by a matrix".to_string())
            }
            (Variable::Matrix(_), Variable::Number(_)) => {
                Err("Cannot pointwise multiply a matrix by a number".to_string())
            }
            (Variable::Matrix(mat1), Variable::Matrix(mat2)) => {
                Ok(Variable::Matrix(mat1.component_mul(&mat2)))
            }
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Variable::Number(num) => write!(f, "{}", num),
            Variable::Matrix(mat) => write!(f, "{}", mat),
        }
    }
}

impl Add<Variable> for Variable {
    type Output = Result<Variable, String>;

    fn add(self, rhs: Variable) -> Self::Output {
        match (self, rhs) {
            (Variable::Number(num1), Variable::Number(num2)) => Ok(Variable::Number(num1 + num2)),
            (Variable::Number(_), Variable::Matrix(_)) => {
                Err("Cannot add a number to a matrix".to_string())
            }
            (Variable::Matrix(_), Variable::Number(_)) => {
                Err("Cannot add a matrix to a number".to_string())
            }
            (Variable::Matrix(mat1), Variable::Matrix(mat2)) => Ok(Variable::Matrix(mat1 + mat2)),
        }
    }
}

impl Sub<Variable> for Variable {
    type Output = Result<Variable, String>;

    fn sub(self, rhs: Variable) -> Self::Output {
        match (self, rhs) {
            (Variable::Number(num1), Variable::Number(num2)) => Ok(Variable::Number(num1 - num2)),
            (Variable::Number(_), Variable::Matrix(_)) => {
                Err("Cannot subtract a matrix from a number".to_string())
            }
            (Variable::Matrix(_), Variable::Number(_)) => {
                Err("Cannot subtract a number from a matrix".to_string())
            }
            (Variable::Matrix(mat1), Variable::Matrix(mat2)) => Ok(Variable::Matrix(mat1 - mat2)),
        }
    }
}

impl Mul<Variable> for Variable {
    type Output = Result<Variable, String>;

    fn mul(self, rhs: Variable) -> Self::Output {
        match (self, rhs) {
            (Variable::Number(num1), Variable::Number(num2)) => Ok(Variable::Number(num1 * num2)),
            (Variable::Number(num1), Variable::Matrix(mat2)) => Ok(Variable::Matrix(num1 * mat2)),
            (Variable::Matrix(_), Variable::Number(_)) => {
                Err("Cannot multiply a matrix by a scalar".to_string())
            }
            (Variable::Matrix(mat1), Variable::Matrix(mat2)) => Ok(Variable::Matrix(mat1 * mat2)),
        }
    }
}

impl Div<Variable> for Variable {
    type Output = Result<Variable, String>;

    fn div(self, rhs: Variable) -> Self::Output {
        match (self, rhs) {
            (Variable::Number(num1), Variable::Number(num2)) => Ok(Variable::Number(num1 / num2)),
            (Variable::Number(num1), Variable::Matrix(mat2)) => Ok(Variable::Matrix(
                num1 * mat2
                    .try_inverse()
                    .ok_or("Could not invert matrix".to_string())?,
            )),
            (Variable::Matrix(_), Variable::Number(_)) => {
                Err("Cannot divide a matrix by a number".to_string())
            }
            (Variable::Matrix(mat1), Variable::Matrix(mat2)) => Ok(Variable::Matrix(
                mat1 * mat2
                    .try_inverse()
                    .ok_or("Could not invert matrix".to_string())?,
            )),
        }
    }
}
