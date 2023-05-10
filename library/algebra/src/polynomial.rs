use crate::fraction::Fraction;
use std::fmt;
pub struct Polynomial {
    coefficients: Box<Vec<Fraction>>
}

impl Polynomial {
    pub fn new(coefficients: Vec<Fraction>) -> Self {
        Self { coefficients: Box::new(coefficients) }
    }

    pub fn new_identity(len: usize) -> Self {
        Self::new(vec![Fraction::new_num(1); len])
    }

    pub fn len(&self) -> usize {
        self.coefficients.len()
    }

    pub fn get_coefficient(&self, index: usize) -> Fraction {
        self.get_coefficients()[index].clone()
    }

    pub fn get_coefficients(&self) -> Vec<Fraction> {
        *self.coefficients.clone()
    }

    pub fn set_elem(&mut self, index: usize, val: Fraction) -> &mut Self {
        self.coefficients[index] = val;
        self
    }
    
    pub fn calculate(&self, coefficient: Fraction) -> Fraction {
        let mut res = Fraction::default();

        res += Fraction::default();

        let mut exp = self.len() as u32;

        for coef in self.get_coefficients() {
            exp -= 1;
            res += coef * coefficient.to_pow(exp);
        }

        res
    }

    pub fn gorner(&self, coefficient: Fraction) -> Self{
        let mut res = vec![Fraction::default(); self.len()];

        res[0] = self.get_coefficient(0);

        for i in 1..self.len(){
            res[i] = res[i-1].clone()*coefficient.clone() + self.get_coefficient(i);
        }

        Self::new(res)
    }
}

impl fmt::Debug for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut res = String::new();

        let mut exp = self.len() as u32;

        for coef in self.get_coefficients() {
            exp -= 1;
            res = format!("{}\t{:?}x^{}", res, coef, exp);
        }

        write!(f, "{}", res)
        
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len(){
            false
        }
        else {
            for i in 0..self.len() {
                if self.get_coefficient(i) != other.get_coefficient(i) {
                    return false;
                }
            }

            true
        }

    }
}