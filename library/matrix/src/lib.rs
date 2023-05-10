use algebra::fraction::Fraction;
use std::{fmt, ops::Mul};

pub mod determinant;
pub mod inverse;
pub mod fundamental_system_solution;
pub mod characteristic_polynomial;

pub struct Matrix
{
    row: Box<u8>,
    col: Box<u8>,
    elems: Box<Vec<Vec<Fraction>>>
}

impl Matrix {
    pub fn new(arg: Vec<Vec<Fraction>>) -> Self {
        Self {
            row: Box::new(arg.len() as u8),
            col: Box::new(arg[0].len() as u8),
            elems: Box::new(arg)
        }
    }

    pub fn new_zero(row: u8, col: u8) -> Self {
        Self::new(vec![vec![Fraction::default(); col as usize]; row as usize])
    }

    pub fn new_identity(size: u8) -> Self {
        let mut identity = Self::new_zero(size, size);

        for i in 0..(size as usize) {
            identity.elems[i][i] = Fraction::new_num(1);
        }

        identity
    }

    pub fn new_scalar(val: Fraction, size: u8) -> Self {
        let mut scalar = Self::new_zero(size, size);

        for i in 0..(size as usize) {
            scalar.elems[i][i] = val.clone();
        }

        scalar
    }

    pub fn get_row(&self) -> u8 {
        *self.row
    }

    pub fn get_col(&self) -> u8 {
        *self.col
    }

    pub fn get_minor(&self, indexes: Vec<u8>) -> Self {
        let mut res = Self::new_identity(indexes.len() as u8);

        for row in (0..indexes.len() as u8){
            for col in (0..indexes.len() as u8){
                res.set_elem(row, col, self.get_elem(indexes[row as usize], indexes[col as usize]));
            }
        }

        res
    }

    pub fn get_elem(&self, row: u8, col: u8) -> Fraction {
        self.elems[row as usize][col as usize].clone()
    }

    pub fn set_elem(&mut self, row: u8, col: u8, val: Fraction) -> &mut Self {
        self.elems[row as usize][col as usize] = val;

        self
    }

    pub fn transform(&self) -> Self {
        let mut res = Self::new_zero(*self.col, *self.row);

        for r in 0..(*res.row as usize) {
            for c in 0..(*res.col as usize) {
                res.elems[r][c] = self.elems[c][r].clone();
            }
        }

        res
    }

    pub fn det(&self) -> Fraction {
        if self.get_row() != self.get_col() {
            panic!("row != col");
        }

        match self.get_row() {
            1 => self.get_elem(0, 0),
            2 => determinant::det2(self.clone()).to_cut(),
            3 => determinant::det3(self.clone()).to_cut(),
            _ => determinant::gauss(self.clone()).to_cut()
        }
    }

    pub fn swap_rows(&mut self, row1: u8, row2: u8) -> &mut Self{

        for colum in 0..(*self.col as usize) {
            (self.elems[row1 as usize][colum], self.elems[row2 as usize][colum]) = (self.elems[row2 as usize][colum].clone(), self.elems[row1 as usize][colum].clone())
        }

        self
    }

    pub fn swap_columns(&mut self, col1: u8, col2: u8) -> &mut Self{

        for row in 0..(*self.row as usize) {
            (self.elems[row][col1 as usize], self.elems[row][col2 as usize]) = (self.elems[row][col2 as usize].clone(), self.elems[row][col1 as usize].clone())
        } 

        self
    }

    pub fn row_mul_frac(&mut self, row: u8, frac: Fraction) -> &mut Self{

        for colum in 0..(*self.col as usize) {
            self.elems[row as usize][colum] *= frac.clone();
            self.elems[row as usize][colum].cut();
        }

        self
    }

    pub fn col_mul_frac(&mut self, col: u8, frac: Fraction) -> &mut Self{

        for row in 0..(*self.row as usize) {
            self.elems[row][col as usize] *= frac.clone();
            self.elems[row][col as usize].cut();
        }

        self
    }

    pub fn row_add_row_mul_frac(&mut self, row1: u8, row2: u8, frac: Fraction) -> &mut Self{

        for colum in 0..(*self.col as usize) {
            let a = self.elems[row2 as usize][colum].clone();
            self.elems[row1 as usize][colum] += a * frac.clone();
            self.elems[row1 as usize][colum].cut();
        }

        self
    }

    pub fn col_add_col_mul_frac(&mut self, col1: u8, col2: u8, frac: Fraction) -> &mut Self{

        for row in 0..(*self.row as usize) {
            let a = self.elems[row][col2 as usize].clone();
            self.elems[row][col1 as usize] += a * frac.clone();
            self.elems[row][col1 as usize].cut();
        }

        self
    }

    pub fn to_string(&self) -> String {
        let mut res = String::new();

        for row in *self.elems.clone() {
            for col in row {
                res = format!("{}{:?}\t", res, col);
            }
            res = format!("{}\n", res);
        }

        res
    }

}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Self {
        Self::new(*self.elems.clone())
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut res = Matrix::new_zero(self.get_row(), other.get_col());

        for row in 0..res.get_row() {
            for col in 0..res.get_col() {
                for s in 0..self.get_col() {
                    res.set_elem(row, col, 
                        self.get_elem(row, s) * other.get_elem(s, col) + res.get_elem(row, col)
                    );
                }
            }
        }

        res
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.get_row() != other.get_row() || self.get_col() != other.get_col() {
            false
        }
        else {
            for row in 0..self.get_row() {
                for col in 0..self.get_col() {
                    if self.get_elem(row, col) != other.get_elem(row, col)
                    {
                        return false;
                    }
                }
            }

            true
        }

    }
}