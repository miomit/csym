use crate::nod;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

pub struct Fraction
{
    numerator: Box<i64>,
    denominator: Box<i64>
}

impl Fraction{
    pub fn new(mut numerator: i64, mut denominator: i64) -> Self {

        if denominator < 0{
            denominator *= -1;
            numerator *= -1;
        }

        Self {
            numerator: Box::new(numerator),
            denominator: Box::new(denominator)
        }
    }

    pub fn new_num(numerator: i64) -> Self{
        Self::new(numerator, 1)
    }

    pub fn to_string(&self) -> String {
        if *self.denominator != 1{
            format!("{}/{}", self.numerator, self.denominator)
        } else {
            format!("{}", self.numerator)
        }
    }

    pub fn cut(&mut self) -> &mut Self {
        let nod = nod(*self.numerator, *self.denominator);

        *self.numerator /= nod;
        *self.denominator /= nod;

        self
    }

    pub fn to_cut(&self) -> Self {
        let nod = nod(*self.numerator, *self.denominator);

        Self::new(*self.numerator / nod, *self.denominator / nod)
    }

    pub fn neg(&mut self) -> &mut Self {
        *self.numerator *= -1;

        self
    }

    pub fn to_neg(&self) -> Self {
        Self::new(*self.numerator * -1_i64, *self.denominator)
    }

    pub fn inv(&mut self) -> &mut Self {
        (*self.numerator, *self.denominator) = (self.get_denominator(), self.get_numerator());

        self
    }

    pub fn to_inv(&self) -> Self {
        Self::new(self.get_denominator(), self.get_numerator())
    }

    pub fn get_result(&self) -> f64{
        (*self.numerator as f64) / (*self.denominator as f64)
    }

    pub fn get_numerator(&self) -> i64{
        *self.numerator
    }

    pub fn get_denominator(&self) -> i64{
        *self.denominator
    }

    pub fn is_zero(&self) -> bool {
        if self.get_numerator() == 0 { true }else { false }
    }
}

impl Default for Fraction {
    fn default() -> Self {
        Fraction::new_num(0)
    }
}

impl Clone for Fraction {
    fn clone(&self) -> Self {
        Self {
            numerator: Box::new(*self.numerator),
            denominator: Box::new(*self.denominator)
        }
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        *self.numerator * *other.denominator == *self.denominator * *other.numerator
    }
}

impl fmt::Debug for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self.denominator != 1{
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
        else{
            write!(f, "{}", self.numerator)
        }
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Fraction::new(*self.numerator * *other.denominator + *other.numerator * *self.denominator,
                      *self.denominator * *other.denominator).to_cut()
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Fraction::new(*self.numerator * *other.denominator - *other.numerator * *self.denominator,
                      *self.denominator * *other.denominator).to_cut()
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Fraction::new(*self.numerator * *other.numerator, *self.denominator * *other.denominator).to_cut()
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Fraction::new(*self.numerator * *other.denominator, *self.denominator * *other.numerator).to_cut()
    }
}

impl std::ops::MulAssign for Fraction {
    fn mul_assign(&mut self, other: Self) {
        *self.numerator *= *other.numerator;
        *self.denominator *= *other.denominator;
    }
}

impl std::ops::AddAssign for Fraction {
    fn add_assign(&mut self, other: Self) {
        *self.numerator = *self.numerator * *other.denominator + *self.denominator * *other.numerator;
        *self.denominator *= *other.denominator;
    }
}