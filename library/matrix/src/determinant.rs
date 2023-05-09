use crate::Matrix;
use crate::Fraction;

pub fn det2(matrix : Matrix) -> Fraction {
        matrix.get_elem(0, 0) * matrix.get_elem(1, 1)
    -   matrix.get_elem(0, 1) * matrix.get_elem(1, 0)
}

pub fn det3(matrix : Matrix) -> Fraction {

        matrix.get_elem(0, 0) * matrix.get_elem(1, 1) * matrix.get_elem(2, 2)
    +   matrix.get_elem(0, 1) * matrix.get_elem(1, 2) * matrix.get_elem(2, 0)
    +   matrix.get_elem(0, 2) * matrix.get_elem(1, 0) * matrix.get_elem(2, 1)

    -   matrix.get_elem(0, 2) * matrix.get_elem(1, 1) * matrix.get_elem(2, 0)
    -   matrix.get_elem(0, 0) * matrix.get_elem(1, 2) * matrix.get_elem(2, 1)
    -   matrix.get_elem(0, 1) * matrix.get_elem(1, 0) * matrix.get_elem(2, 2)
}

pub fn gauss(mut matrix: Matrix) -> Fraction {
    
    let mut lambda = Fraction::new_num(1);
    
    for dioganal in 0..=(matrix.get_row() - 1_u8) {
        if matrix.get_elem(dioganal, dioganal).is_zero() {
            let mut row_no_zero_elem: Option<u8> = None;

            for i in 1..(matrix.get_row() - dioganal) {
                
                if !matrix.get_elem(dioganal + i, dioganal).is_zero() {
                    row_no_zero_elem = Some(dioganal+i);
                    break;
                }
            }

            if let Some(row) = row_no_zero_elem {
                lambda.neg();
                matrix.swap_rows(dioganal, row);
            } else {
                lambda = Fraction::new_num(0);
                break;
            }
            
        }

        let inverse_diagonal_element = matrix.get_elem(dioganal, dioganal).to_inv();

        matrix.row_mul_frac(dioganal, inverse_diagonal_element.clone());

        lambda *= inverse_diagonal_element;

        for i in 1..(matrix.get_row() - dioganal) {
            matrix.row_add_row_mul_frac(dioganal + i, dioganal, matrix.get_elem(dioganal + i, dioganal).to_neg());
        }
    }

    lambda.to_inv()
}