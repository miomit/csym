use crate::Matrix;
use crate::Fraction;

use algebra::polynomial::Polynomial;
use discrete;

pub fn invariant(matrix: Matrix) -> Polynomial {
    let mut res = Polynomial::new_identity((matrix.get_row() + 1) as usize);

    for i in 1..(res.len() -1) {
        let mut sum = Fraction::default();
        for prerm in discrete::perm_ascending_and_without_repetitions(res.len()-1, i) {
            sum += matrix.get_minor(prerm).det();
        }

        if i != 2{
            if i % 2 == 1{
                sum.neg();
            }
        } 

        res.set_elem( i, sum.clone());
    }

    if res.len() % 2 == 0{
        res.set_elem(0, Fraction::new_num(-1));
    }

    res.set_elem(res.len() - 1, matrix.det());

    res
}