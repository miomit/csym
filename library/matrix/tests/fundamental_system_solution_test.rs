use algebra::fraction::Fraction;
use matrix::{Matrix, inverse};

#[test]
fn gauss() {
    let matrix = Matrix::new(vec![
        vec![Fraction::new_num(1), Fraction::new_num(0), Fraction::new_num(0), Fraction::new_num(-2), Fraction::new_num(1)],
        vec![Fraction::new_num(3), Fraction::new_num(-1), Fraction::new_num(0), Fraction::new_num(-1), Fraction::new_num(0)],
        vec![Fraction::new_num(0), Fraction::new_num(0), Fraction::new_num(-1), Fraction::new_num(1), Fraction::new_num(0)],
    ]);

    let a = matrix::fundamental_system_solution::gauss(matrix.clone());

    let res = Matrix::new_zero(3, 1);

    assert_eq!(a.len(), 2);    
    assert_eq!(matrix.clone() * a[0].clone(), res);
    assert_eq!(matrix.clone() * a[1].clone(), res);

    let matrix = Matrix::new(vec![
        vec![Fraction::new_num(1), Fraction::new_num(1), Fraction::new_num(-5), Fraction::new_num(-7)],
        vec![Fraction::new_num(2), Fraction::new_num(1), Fraction::new_num(4), Fraction::new_num(1)],
        vec![Fraction::new_num(3), Fraction::new_num(2), Fraction::new_num(-1), Fraction::new_num(-6)],
    ]);

    let a = matrix::fundamental_system_solution::gauss(matrix.clone());

    let res = Matrix::new_zero(3, 1);

    assert_eq!(a.len(), 2);    
    assert_eq!(matrix.clone() * a[0].clone(), res);
    assert_eq!(matrix.clone() * a[1].clone(), res);
}