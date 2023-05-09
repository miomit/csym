pub mod fraction;

pub fn nod(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a.abs()
}

pub fn nok(a: i64, b: i64) -> i64 {
    ((a/ nod(a, b)) * b).abs()
}