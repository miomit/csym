use std::vec;

use algebra::fraction::Fraction;

use crate::Matrix;

pub fn gauss(mut matrix: Matrix) -> Vec<Matrix> {

    let mut col = 0;
    
    let mut main_colums: Vec<u8> = vec![];
    let mut main_rows: Vec<u8> = vec![];

    for row in 0..matrix.get_row() {
        if col == matrix.get_col() {
            break;
        }

        if matrix.get_elem(row, col).is_zero() {
            let is_break = loop {
                let mut row_no_zero_elem: Option<u8> = None;
                if col == matrix.get_col() {
                    break true;
                }

                if !matrix.get_elem(row, col).is_zero() {
                    break false;
                }

                for i in 1..(matrix.get_row() - row) {
                    
                    if !matrix.get_elem(row + i, col).is_zero() {
                        row_no_zero_elem = Some(row+i);
                    }
                }

                if let Some(r) = row_no_zero_elem {
                    println!("swap_rows {} {}; col = {}", row, r, col);
                    matrix.swap_rows(row, r);
                    break false;
                } else {
                    col += 1;
                }
            };

            if is_break {
                break;
            }
            
        }

        main_rows.push(row);
        main_colums.push(col);
        
        let inverse_diagonal_element = matrix.get_elem(row, col).to_inv();
        matrix.row_mul_frac(row, inverse_diagonal_element);

        for i in 1..(matrix.get_row() - row) {
            matrix.row_add_row_mul_frac(row + i, row, matrix.get_elem(row + i, col).to_neg());
        }

        matrix.row_mul_frac(row, Fraction::new_num(-1));

    }
    
    let df = matrix.get_col() - main_colums.len() as u8;

    let identity = Matrix::new_identity(df);

    let mut res_bases: Vec<Vec<Option<Fraction>>> = vec![vec![None; matrix.get_col() as usize]; df as usize];

    let free_colums: Vec<u8> = difference(&(0..matrix.get_col()).collect::<Vec<u8>>(), &main_colums);

    for row in 0..df {
        for col in 0..free_colums.len() {
            res_bases[row as usize][free_colums[col] as usize] = Some(identity.get_elem(row, col as u8));
        }
    }

    let mut free_row = df - 1;
    for i in (0..df).rev(){
        let mut index_col = main_colums.len();
        for row in main_rows.iter().rev() {
            index_col -= 1;
            let main_col = main_colums[index_col];
            if res_bases[i as usize][main_col as usize].is_some() {
                panic!("fss error step");
            }
            for col in (main_col+1)..matrix.get_col(){
                if let Some(elem) = res_bases[i as usize][main_col as usize].clone() {
                    res_bases[i as usize][main_col as usize] = Some(elem + matrix.get_elem(*row, col) * res_bases[i as usize][col as usize].clone().unwrap());
                } else {
                    res_bases[i as usize][main_col as usize] = Some(matrix.get_elem(*row, col) * res_bases[i as usize][col as usize].clone().unwrap());
                }
            }
        }
    }

    let mut res = vec![Matrix::new_zero(matrix.get_col(), 1); df as usize];

    for i in 0..df {
        for j in 0..matrix.get_col(){
            res[i as usize].set_elem(j, 0, res_bases[i as usize][j as usize].clone().unwrap());
        }
    }

    res
}

fn difference(v1: &[u8], v2: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    for &x in v1 {
        if !v2.contains(&x) {
            result.push(x);
        }
    }
    result
} 