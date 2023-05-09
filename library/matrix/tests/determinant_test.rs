use algebra::fraction::Fraction;
use matrix::{Matrix, determinant};

#[test]
fn det2() {
    let matrix = Matrix::new(vec![
        vec![Fraction::new_num(1), Fraction::new_num(2)],
        vec![Fraction::new_num(1), Fraction::new_num(2)]
    ]);

    let res = Fraction::default();
    assert_eq!(determinant::det2(matrix), res);

    let matrix = Matrix::new(vec![
        vec![Fraction::new_num(2), Fraction::new_num(1)],
        vec![Fraction::new_num(1), Fraction::new_num(2)]
    ]);

    let res = Fraction::new_num(3);
    assert_eq!(determinant::det2(matrix), res);

    let matrix = Matrix::new(vec![
        vec![Fraction::new_num(0), Fraction::new_num(-7)],
        vec![Fraction::new_num(-5), Fraction::new_num(2)]
    ]);

    let res = Fraction::new_num(-35);
    assert_eq!(determinant::det2(matrix), res);
}

#[test]
fn det3() {
    let matrix = Matrix::new(vec![
        vec![Fraction::new_num(2), Fraction::new_num(-2), Fraction::new_num(3)],
        vec![Fraction::new_num(3), Fraction::new_num(4), Fraction::new_num(5)],
        vec![Fraction::new_num(2), Fraction::new_num(1),Fraction::new_num(-4)]
    ]);

    let res = Fraction::new_num(-101);
    assert_eq!(determinant::det3(matrix), res);
}

#[test]
fn gauss() {
    let matrix = Matrix::new(vec![
        vec![Fraction::new_num(0), Fraction::new_num(-7)],
        vec![Fraction::new_num(-5), Fraction::new_num(2)]
    ]);

    let res = Fraction::new_num(-35);
    assert_eq!(determinant::gauss(matrix), res);

    let matrix = Matrix::new(vec![
        vec![Fraction::new_num(2), Fraction::new_num(-2), Fraction::new_num(3)],
        vec![Fraction::new_num(3), Fraction::new_num(4), Fraction::new_num(5)],
        vec![Fraction::new_num(2), Fraction::new_num(1),Fraction::new_num(-4)]
    ]);

    let res = Fraction::new_num(-101);
    assert_eq!(determinant::gauss(matrix), res);

    let matrix = Matrix::new(vec![
        vec![Fraction::new_num(3), Fraction::new_num(-1), Fraction::new_num(2), Fraction::new_num(3)],
        vec![Fraction::new_num(1), Fraction::new_num(-2), Fraction::new_num(3), Fraction::new_num(1)],
        vec![Fraction::new_num(1), Fraction::new_num(3), Fraction::new_num(3), Fraction::new_num(2)],
        vec![Fraction::new_num(1), Fraction::new_num(-1), Fraction::new_num(-2), Fraction::new_num(0)]
    ]);

    let res = Fraction::new_num(17);
    assert_eq!(determinant::gauss(matrix), res);

    let matrix = Matrix::new(vec![
        vec![Fraction::new_num(2), Fraction::new_num(1), Fraction::new_num(2), Fraction::new_num(3)],
        vec![Fraction::new_num(1), Fraction::new_num(5), Fraction::new_num(3), Fraction::new_num(9)],
        vec![Fraction::new_num(3), Fraction::new_num(1), Fraction::new_num(0), Fraction::new_num(0)],
        vec![Fraction::new(1, 2), Fraction::new(25, 10), Fraction::new(3, 2), Fraction::new(1, 10)]
    ]);

    let res = Fraction::new_num(110);
    assert_eq!(determinant::gauss(matrix), res);
}