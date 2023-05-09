use crate::Matrix;

pub fn gauss_jordan(mut matrix: Matrix) -> Matrix{
    let mut res = Matrix::new_identity(matrix.get_row());

    for dioganal in 0..matrix.get_row() {
        if matrix.get_elem(dioganal, dioganal).is_zero() {
            let mut row_no_zero_elem: Option<u8> = None;

            for i in 1..(matrix.get_row() - dioganal) {
                
                if !matrix.get_elem(dioganal + i, dioganal).is_zero() {
                    row_no_zero_elem = Some(dioganal+i);
                    break;
                }
            }

            if let Some(row) = row_no_zero_elem {
                matrix.swap_rows(dioganal, row);
                res.swap_rows(dioganal, row);
            }
            
        }

        let inverse_diagonal_element = matrix.get_elem(dioganal, dioganal).to_inv();

        res.row_mul_frac(dioganal, inverse_diagonal_element.clone());
        matrix.row_mul_frac(dioganal, inverse_diagonal_element.clone());

        for i in 1..(matrix.get_row() - dioganal) {
            res.row_add_row_mul_frac(dioganal + i, dioganal, matrix.get_elem(dioganal + i, dioganal).to_neg());
            matrix.row_add_row_mul_frac(dioganal + i, dioganal, matrix.get_elem(dioganal + i, dioganal).to_neg());
        }
    }

    for dioganal in (0..matrix.get_row()).rev() {
        if matrix.get_elem(dioganal, dioganal).is_zero() {
            panic!("❗❗❗There is no inverse matrix");
        }

        for i in 1..=dioganal {
            res.row_add_row_mul_frac(dioganal - i, dioganal, matrix.get_elem(dioganal - i, dioganal).to_neg());
            matrix.row_add_row_mul_frac(dioganal - i, dioganal, matrix.get_elem(dioganal - i, dioganal).to_neg());
        }
    }

    res
}