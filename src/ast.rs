#![allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn from_tuples(tuples: Vec<(f64, Option<String>)>) -> Self {
        let mut rows: Vec<Vec<f64>> = Vec::new();
        let mut current_row: Vec<f64> = Vec::new();
        for (num, semi) in tuples {
            current_row.push(num);

            if let Some(_) = semi {
                rows.push(current_row);
                current_row = Vec::new();
            }
        }
        rows.push(current_row);

        Self { rows }
        // let mut data: Vec<f64> = Vec::with_capacity(tuples.len());
        // let mut rows: usize = if tuples.len() > 0 { 1 } else { 0 };
        // let mut columns: usize = 0;
        // let mut column: usize = 0;
        // for (num, semi) in tuples {
        //     data.push(num);
        //     column += 1;

        //     if let Some(_) = semi {
        //         if columns > 0 && column != columns {
        //             return Err("Columns don't match".to_string());
        //         }
        //         columns = column;
        //         column = 0;
        //         rows += 1;
        //     }
        // }
        // if columns > 0 && column != columns {
        //     return Err("Columns don't match".to_string());
        // }

        // Ok(Self {
        //     rows,
        //     columns,
        //     data,
        // })
    }
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Matrix(Matrix),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Value(Value),
    BinaryExpr(Box<Expr>, Op, Box<Expr>),
    Ident(String),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    AssignmentStmt(String, Expr),
}
