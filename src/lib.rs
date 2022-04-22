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

struct Functions<'a> {
    functions: HashMap<String, &'a dyn Fn(Vec<Variable>) -> Result<Variable, String>>,
}

#[wasm_bindgen]
pub struct Ratlab {
    parser: StmtParser,
    variables: HashMap<String, Variable>,
    functions: Functions<'static>,
}

fn zeros(variables: Vec<Variable>) -> Result<Variable, String> {
    if variables.len() != 2 {
        return Err("Zeros must be called with 2 variables.".into());
    }

    let rows = (&variables[0]).clone();
    let columns = (&variables[1]).clone();
    let rows = match rows {
        Variable::Number(num) => {
            if num.floor() == num {
                num as usize
            } else {
                return Err("Rows cannot be a float".into());
            }
        }
        Variable::Matrix(_) => return Err("Argument cannot be a matrix".into()),
    };
    let columns = match columns {
        Variable::Number(num) => {
            if num.floor() == num {
                num as usize
            } else {
                return Err("Rows cannot be a float".into());
            }
        }
        Variable::Matrix(_) => return Err("Argument cannot be a matrix".into()),
    };

    Ok(Variable::Matrix(DMatrix::<f64>::zeros(rows, columns)))
}

#[wasm_bindgen]
impl Ratlab {
    pub fn new() -> Self {
        set_panic_hook();

        let parser = StmtParser::new();
        let variables = HashMap::new();
        let mut functions: HashMap<String, &dyn Fn(Vec<Variable>) -> Result<Variable, String>> =
            HashMap::new();

        functions.insert("zeros".to_string(), &zeros);

        let functions = Functions { functions };

        Self {
            parser,
            variables,
            functions,
        }
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
                    Op::PointwiseMul => lhs.pointwise_mul(rhs),
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
            Expr::Transpose(expr) => {
                let expr = *expr;
                let to_transpose = self.parse_expr(expr)?;
                match to_transpose {
                    Variable::Number(_) => Err("Cannot transpose a number".to_string()),
                    Variable::Matrix(mat) => Ok(Variable::Matrix(mat.transpose())),
                }
            }
            Expr::CallExpr(func_name, args) => {
                let mut variables: Vec<Variable> = Vec::with_capacity(args.len());
                for expr in args {
                    let expr = *expr;
                    let variable = self.parse_expr(expr)?;
                    variables.push(variable);
                }

                self.call_func(func_name, variables)
            }
        }
    }

    fn call_func(&mut self, func_name: String, args: Vec<Variable>) -> Result<Variable, String> {
        println!("keys: {:?}", self.functions.functions.keys());
        if let Some(func) = self.functions.functions.get(&func_name) {
            func(args)
        } else {
            Err(format!("{} is not defined", func_name))
        }
    }

    pub fn input(&mut self, input: &str) -> Result<String, String> {
        let ast = self.parser.parse(input).map_err(|err| err.to_string())?;
        println!("ast: {:?}", ast);

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
