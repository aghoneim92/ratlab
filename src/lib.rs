mod ast;
mod utils;
mod variable;

use crate::utils::set_panic_hook;
use std::collections::HashMap;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

use ast::{Expr, Matrix, Op, Stmt};
use grammar::StmtParser;
use nalgebra::{DMatrix, RowDVector};
use variable::Variable;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(input: &str);
}

#[wasm_bindgen]
pub struct Ratlab {
    parser: StmtParser,
    variables: HashMap<String, Variable>,
}

#[wasm_bindgen]
impl Ratlab {
    pub fn new() -> Self {
        set_panic_hook();

        let parser = StmtParser::new();
        let variables = HashMap::new();

        Self { parser, variables }
    }

    fn d_matrix_from_matrix(&mut self, mat: Matrix) -> Result<DMatrix<f64>, String> {
        // let mut columns: usize = 0;
        let rows: usize = mat.rows.len();
        // if rows > 0 {
        //     columns = mat.rows[0].len();
        // }
        let mut row_vectors: Vec<RowDVector<f64>> = Vec::with_capacity(rows);
        for row in mat.rows {
            // if columns != row.len() {
            //     return Err("Rows don't all have the same size".to_string());
            // }
            let mut row_vec: Vec<f64> = vec![];
            for expr in row {
                let value = self.parse_expr(*expr)?;
                match value {
                    Variable::Number(num) => {
                        row_vec.push(num);
                    }
                    Variable::Matrix(mat) => {
                        if mat.shape().0 > 1 {
                            return Err("nope".to_string());
                        }
                        for i in 0..mat.shape().1 {
                            row_vec.push(mat[(0, i)]);
                        }
                    }
                }
            }
            let row_vector = RowDVector::from_vec(row_vec);
            row_vectors.push(row_vector);
        }

        Ok(DMatrix::from_rows(&row_vectors[..]))
    }

    fn parse_expr(&mut self, expr: Expr) -> Result<Variable, String> {
        match expr {
            Expr::Value(value) => Ok(match value {
                ast::Value::Number(num) => Variable::Number(num),
                ast::Value::Matrix(mat) => Variable::Matrix(self.d_matrix_from_matrix(mat)?),
            }),
            Expr::BinaryExpr(lhs, op, rhs) => {
                let lhs = self.parse_expr(*lhs)?;
                let rhs = self.parse_expr(*rhs)?;

                match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                }
            }
            Expr::Ident(ident) => {
                let variable = self.variables.get(&ident);

                if let Some(variable) = variable {
                    Ok(variable.clone())
                } else {
                    Err(format!("{} is not defined", ident))
                }
            }
            Expr::Range(start, end) => Ok(Variable::Matrix(DMatrix::from_iterator(
                1usize,
                (end - start + 1.0) as usize,
                (start as i64..=end as i64)
                    .into_iter()
                    .map(|val| val as f64),
            ))),
        }
    }

    pub fn input(&mut self, input: &str) -> Result<String, String> {
        let ast = self.parser.parse(input).map_err(|err| err.to_string())?;

        Ok(match ast {
            Stmt::ExprStmt(expr) => self.parse_expr(expr)?.to_string(),
            Stmt::AssignmentStmt(ident, expr) => {
                let expr = self.parse_expr(expr)?;
                self.variables.insert(ident, expr.clone());
                expr.to_string()
            }
        })
    }
}
