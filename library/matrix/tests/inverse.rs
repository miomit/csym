use algebra::fraction::Fraction;
use matrix::{Matrix, inverse};

#[test]
fn gauss_jordan() {

    let matrix = Matrix::new(vec![
        vec![Fraction::new_num(3), Fraction::new_num(1), Fraction::new_num(4)],
        vec![Fraction::new_num(2), Fraction::new_num(1231), Fraction::new_num(6)],
        vec![Fraction::new_num(3), Fraction::new_num(3), Fraction::new_num(9)]
    ]);

    let inv = inverse::gauss_jordan(matrix.clone());
    
    assert_eq!(matrix.clone() * inv.clone(), Matrix::new_identity(3));

    let matrix = Matrix::new(vec![
        vec![Fraction::new_num(-21), Fraction::new_num(1)],
        vec![Fraction::new_num(3), Fraction::new_num(2)],
    ]);

    let inv = inverse::gauss_jordan(matrix.clone());
    
    assert_eq!(matrix.clone() * inv.clone(), Matrix::new_identity(2));
}