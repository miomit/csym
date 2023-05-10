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

    pub fn solve(&self) -> Vec<Fraction>{
        let mut res = Vec::new();

        let mut self_short = Polynomial::new(self.get_coefficients().clone());

        for i in (1..self.len()).rev() {
            if self_short.get_coefficient(i).is_zero() {
                res.push(Fraction::default());
            }
            else{
                let mut k = self_short.get_coefficient(i).get_result();

                if k < 0_f64 { k *= -1_f64;}

                let mut is_found_root = false;

                for j in 1..=( (k + 1_f64) as i64){

                    for num in [Fraction::new_num(j), Fraction::new_num(-j)] {

                        let gorner_pol = self_short.gorner(num.clone());
                        
                        if gorner_pol.get_coefficient(self_short.len() - 1).is_zero() {
                            res.push(num);
                            is_found_root = true;

                            self_short = Polynomial::new(gorner_pol.get_coefficients()[0..i].to_vec());

                            break;
                        }

                    }

                    if is_found_root {
                        break;
                    }
                }

                if !is_found_root {
                    panic!("is_found_root = false :(");
                }
            }
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