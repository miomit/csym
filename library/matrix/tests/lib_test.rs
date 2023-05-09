use algebra::fraction::Fraction;
use matrix::Matrix;



#[test]
fn mul() {
    let a = Matrix::new(vec![
        vec![Fraction::new_num(0), Fraction::new_num(1), Fraction::new_num(2)],
        vec![Fraction::new_num(1), Fraction::new_num(0), Fraction::new_num(1)]
    ]);

    let b = Matrix::new(vec![
        vec![Fraction::new_num(1), Fraction::new_num(2)],
        vec![Fraction::new_num(2), Fraction::new_num(2)],
        vec![Fraction::new_num(3), Fraction::new_num(3)],
    ]);

    let res_a_b = Matrix::new(vec![
        vec![Fraction::new_num(8), Fraction::new_num(8)],
        vec![Fraction::new_num(4), Fraction::new_num(5)]
    ]);

    let res_b_a = Matrix::new(vec![
        vec![Fraction::new_num(2), Fraction::new_num(1), Fraction::new_num(4)],
        vec![Fraction::new_num(2), Fraction::new_num(2), Fraction::new_num(6)],
        vec![Fraction::new_num(3), Fraction::new_num(3), Fraction::new_num(9)]
    ]);

    assert_eq!(a.clone() * b.clone(), res_a_b);
    assert_eq!(b * a, res_b_a);
    
}