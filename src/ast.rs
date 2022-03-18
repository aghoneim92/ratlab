#![allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: Vec<Vec<Box<Expr>>>,
}

impl Matrix {
    pub fn from_tuples(tuples: Vec<(Box<Expr>, Option<String>)>) -> Self {
        let mut rows: Vec<Vec<Box<Expr>>> = Vec::new();
        let mut current_row: Vec<Box<Expr>> = Vec::new();
        for (num, semi) in tuples {
            current_row.push(num);

            if let Some(_) = semi {
                rows.push(current_row);
                current_row = Vec::new();
            }
        }
        rows.push(current_row);

        Self { rows }
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
    Range(f64, f64),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    AssignmentStmt(String, Expr),
}
