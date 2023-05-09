use algebra::fraction::*;

#[test]
fn to_string_tests() {
    let f = Fraction::new(1, 2);
    assert_eq!(f.to_string(), "1/2");
    let f = Fraction::new_num(1);
    assert_eq!(f.to_string(), "1");
}

#[test]
fn to_cut_tests() {
    let l = Fraction::new(2, 2).to_cut();
    let r = Fraction::new_num(1).to_cut();
    assert_eq!(l, r);
    let l = Fraction::new(18, 6).to_cut();
    let r = Fraction::new_num(3).to_cut();
    assert_eq!(l, r);
}

#[test]
fn mul_tests() {
    let a = Fraction::new_num(2);
    let b = Fraction::new_num(3);

    let res = Fraction::new_num(6);

    assert_eq!(a*b, res);

    let a = Fraction::new(2, 3);
    let b = Fraction::new(3, 2);

    let res = Fraction::new_num(1);

    assert_eq!(a*b, res);

    let mut a = Fraction::new(2, 3);
    let b = Fraction::new(3, 2);

    a *= b;

    let res = Fraction::new_num(1);

    assert_eq!(a, res)
}

#[test]
fn div_tests() {
    let a = Fraction::new_num(2);
    let b = Fraction::new_num(3);

    let res = Fraction::new(2,3);

    assert_eq!(a/b, res);

    let a = Fraction::new(2, 3);
    let b = Fraction::new(3, 2);

    let res = Fraction::new(4, 9);

    assert_eq!(a/b, res);
}

#[test]
fn add_tests() {
    let a = Fraction::new_num(2);
    let b = Fraction::new_num(3);

    let res = Fraction::new_num(5);

    assert_eq!(a+b, res);

    let a = Fraction::new(2, 3);
    let b = Fraction::new(3, 2);

    let res = Fraction::new(13, 6);

    assert_eq!(a+b, res);

    let mut a = Fraction::new(2, 3);
    let b = Fraction::new(3, 2);

    a += b;
    let res = Fraction::new(13, 6);

    assert_eq!(a, res);
}

#[test]
fn sub_tests() {
    let a = Fraction::new_num(2);
    let b = Fraction::new_num(3);

    let res = Fraction::new_num(-1);

    assert_eq!(a-b, res);

    let a = Fraction::new(2, 3);
    let b = Fraction::new(3, 2);

    let res = Fraction::new(-5, 6);

    assert_eq!(a-b, res);

    let a = Fraction::new(2, 3);
    let b = Fraction::new(3, -2);

    let res = Fraction::new(13, 6);

    assert_eq!(a-b, res);
}