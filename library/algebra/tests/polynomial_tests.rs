use algebra::fraction::Fraction;
use algebra::polynomial::Polynomial;

#[test]
fn gorner_tests() {
    let a = Polynomial::new(vec![Fraction::new_num(1), Fraction::new_num(1)]);
    let res =  Polynomial::new(vec![Fraction::new_num(1), Fraction::new_num(2)]);

    assert_eq!(a.gorner(Fraction::new_num(1)), res);

    let a = Polynomial::new(vec![Fraction::new_num(2), Fraction::new_num(1), Fraction::new_num(-1)]);
    let res =  Polynomial::new(vec![Fraction::new_num(2), Fraction::new_num(-1), Fraction::new_num(0)]);

    assert_eq!(a.gorner(Fraction::new_num(-1)), res);
}