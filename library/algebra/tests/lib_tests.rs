use algebra::*;

#[test]
fn nod_tests() {
    assert_eq!(nod(2, -3), 1);
    assert_eq!(nod(13, 14), 1);
    assert_eq!(nod(124, 7912), 4);
}

#[test]
fn nok_tests() {
    assert_eq!(nok(2, -3), 6);
    assert_eq!(nok(13, 14), 182);
    assert_eq!(nok(124, 7912), 245272);
}
