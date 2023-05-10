use algebra::fraction::Fraction;
use matrix::Matrix;

#[test]
fn get_minor_tests () {
    let a = Matrix::new(vec![
        vec![Fraction::new_num(0), Fraction::new_num(1), Fraction::new_num(2)],
        vec![Fraction::new_num(1), Fraction::new_num(0), Fraction::new_num(1)]
    ]);

    let res = Matrix::new(vec![
        vec![Fraction::new_num(0), Fraction::new_num(1)],
        vec![Fraction::new_num(1), Fraction::new_num(0)]
    ]);

    assert_eq!(a.get_minor(vec![0, 1]), res);

    let a = Matrix::new(vec![
        vec![Fraction::new_num(2), Fraction::new_num(123), Fraction::new_num(4), Fraction::new_num(4)],
        vec![Fraction::new_num(5), Fraction::new_num(312), Fraction::new_num(43), Fraction::new_num(45)],
        vec![Fraction::new_num(23), Fraction::new_num(321), Fraction::new_num(-4), Fraction::new_num(2)],
        vec![Fraction::new_num(12), Fraction::new_num(43), Fraction::new_num(4), Fraction::new_num(3)],
    ]);

    let res = Matrix::new(vec![
        vec![Fraction::new_num(2), Fraction::new_num(123), Fraction::new_num(4)],
        vec![Fraction::new_num(5), Fraction::new_num(312), Fraction::new_num(45)],
        vec![Fraction::new_num(12), Fraction::new_num(43), Fraction::new_num(3)],
    ]);

    assert_eq!(a.get_minor(vec![0, 1, 3]), res);
}

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