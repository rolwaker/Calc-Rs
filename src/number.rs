use std::mem::swap;

use super::bignum::Bignum;

pub struct Number {
    num: Bignum,
    den: Bignum
}

impl Number {
    pub fn from(digits: &str) -> Option<Number> {
        if let Some(n) = Bignum::from(digits) {
            Some(Number {
                num: n,
                den: Bignum::one()
            })
        } else {
            None
        }
    }
    
    fn reduce(&mut self) {
        let sign = self.num.is_neg() ^ self.den.is_neg();
        
        if self.num.is_neg() {
            self.num.neg();
        }
        
        if self.den.is_neg() {
            self.den.neg();
        }
        
        let mut n = self.num.clone();
        let mut d = self.den.clone();
        
        while !d.is_zero() {
            n.rem(&d);
            swap(&mut n, &mut d);
        }
        
        self.num.div(&n);
        self.den.div(&n);
        
        if sign {
            self.num.neg();
        }
    }
    
    pub fn add(&mut self, oth: &Number) {
        let mut onum = oth.num.clone();
        onum.mul(&self.den);
        
        self.num.mul(&oth.den);
        self.den.mul(&oth.den);
        
        self.num.add(&onum);
        self.reduce();
    }
    
    pub fn sub(&mut self, oth: &Number) {
        let mut onum = oth.num.clone();
        onum.mul(&self.den);
        
        self.num.mul(&oth.den);
        self.den.mul(&oth.den);
        
        self.num.sub(&onum);
        self.reduce();
    }
    
    pub fn neg(&mut self) {
        self.num.neg();
    }
    
    pub fn mul(&mut self, oth: &Number) {
        self.num.mul(&oth.num);
        self.den.mul(&oth.den);
        self.reduce();
    }
    
    pub fn div(&mut self, oth: &Number) -> Result<(), String> {
        if !oth.num.is_zero() {
            self.num.mul(&oth.den);
            self.den.mul(&oth.num);
            self.reduce();
            Ok(())
        } else {
            Err("divide by zero".to_string())
        }
    }
    
    pub fn rem(&mut self, oth: &Number) -> Result<(), String> {
        if !oth.num.is_zero() {
            let mut onum = oth.num.clone();
            onum.mul(&self.den);
            
            self.num.mul(&oth.den);
            self.den.mul(&oth.den);
            
            self.num.rem(&onum);
            self.reduce();
            Ok(())
        } else {
            Err("modulo by zero".to_string())
        }
    }
    
    pub fn pow(&mut self, oth: &Number) -> Result<(), String> {
        // currently only integer powers are supported, in part because
        // arbitrary precision roots are hard but also because irrational
        // numbers and complex values makes everything more complicated.
        if oth.den == Bignum::one() {
            let mut onum = oth.num.clone();
            let mut copy = Number {
                num: Bignum::zero(),
                den: Bignum::zero()
            };
            let mut base = Number {
                num: Bignum::one(),
                den: Bignum::one()
            };
            
            if onum.is_neg() {
                onum.neg();
                swap(&mut self.num, &mut self.den);
            }
            
            swap(self, &mut base);
            
            while !onum.is_zero() {
                if !onum.is_even() {
                    self.mul(&mut base);
                }
                
                copy.clone_from(&mut base);
                base.mul(&mut copy);
                
                onum.shr(1);
                onum.trunc();
            }
            
            Ok(())
        } else {
            Err("non-integer power".to_string())
        }
    }
}

impl Clone for Number {
    fn clone(&self) -> Number {
        Number {
            num: self.num.clone(),
            den: self.den.clone()
        }
    }
    
    fn clone_from(&mut self, oth: &Number) {
        self.num.clone_from(&oth.num);
        self.den.clone_from(&oth.den);
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        let mut n = self.num.clone();
        
        if self.den != Bignum::one() {
            n.div(&self.den);
            format!("{} ({}/{})", n.to_string(), self.num.to_string(), self.den.to_string())
        } else {
            n.to_string()
        }
    }
}
