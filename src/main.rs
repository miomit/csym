use algebra::fraction::Fraction;
use matrix::Matrix;

fn main() {
    let matrix = Matrix::new(vec![
        vec![Fraction::new_num(2), Fraction::new_num(1), Fraction::new_num(2), Fraction::new_num(3)],
        vec![Fraction::new_num(1), Fraction::new_num(5), Fraction::new_num(3), Fraction::new_num(9)],
        vec![Fraction::new_num(3), Fraction::new_num(1), Fraction::new_num(0), Fraction::new_num(0)],
        vec![Fraction::new(1, 2), Fraction::new(25, 10), Fraction::new(3, 2), Fraction::new(1, 10)]
    ]);

    println!("{:?}", matrix.det())
}
