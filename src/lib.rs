mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Matrix {
    rows: usize,
    columns: usize,
    data: Vec<f64>,
}

#[wasm_bindgen]
impl Matrix {
    pub fn new(rows: usize, columns: usize, data: Vec<f64>) -> Self {
        Self {
            rows,
            columns,
            data,
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.rows {
            for j in 0..self.columns {
                write!(f, "{}", self.data[i * self.columns + j])?;

                if j != self.columns - 1 {
                    write!(f, ", ")?;
                }
            }

            if i != self.rows - 1 {
                write!(f, "; ")?;
            }
        }
        write!(f, "]")?;

        Ok(())
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, ratlab!");
}
