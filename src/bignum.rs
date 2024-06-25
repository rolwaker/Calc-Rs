use std::slice::Iter;
use std::fmt::Debug;

pub mod comp;
pub mod prod;
pub mod shift;
pub mod string;
pub mod term;
mod tests;

// number of bits and types used to encode values, these should be as small
// as possible for testing and as large as the architecture can easily handle
// when actually being used.
const BITS: usize = 32;
type  Uint = u32;
type  Big  = u64;

pub struct Bignum {
    data: Vec<Uint>,
    sign: bool,
}

struct BnIter<'l> {
    iter: Iter<'l, Uint>,
    cnt: usize,
    max: usize,
    pad: Uint
}

impl Bignum {
    pub fn zero() -> Bignum {
        Bignum {
            data: Vec::from([0, 0, 0, 0]),
            sign: false
        }
    }
    
    pub fn one() -> Bignum {
        Bignum {
            data: Vec::from([0, 0, 0, 0, 1]),
            sign: false
        }
    }
    
    pub fn from(digits: &str) -> Option<Bignum> {
        let mut iter = digits.chars();
        let mut bn = Bignum::zero();
        let mut n2 = Bignum::zero();
        let mut sign = false;
        
        loop {
            match iter.next() {
                Some(c) => {
                    if c.is_digit(10) {
                        bn.shl(1);
                        
                        n2.clone_from(&bn);
                        bn.shl(2);
                        
                        bn.add(&n2);
                        bn._addi(c.to_digit(10).unwrap() as Uint);
                    } else if c == '-' {
                        if !sign {
                            sign = true;
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                },
                None => {
                    bn.shl(4 * BITS);
                    
                    if sign {
                        bn.neg();
                    }
                    
                    return Some(bn)
                }
            }
        }
    }
    
    fn grow_to(&mut self, len: usize) {
        if self.data.len() < len {
            self.data.resize(len, self.pad());
        }
    }
    
    fn grow_by(&mut self, extra: usize) {
        self.data.resize(self.data.len() + extra, self.pad());
    }
    
    fn shrink(&mut self) {
        while self.data.len() > 4 && *self.data.last().unwrap() == self.pad() {
            self.data.pop();
        }
    }
    
    fn pad(&self) -> Uint {
        if self.sign {
            !0
        } else {
            0
        }
    }
    
    pub fn trunc(&mut self) {
        self.data[0] = 0;
        self.data[1] = 0;
        self.data[2] = 0;
        self.data[3] = 0;
    }
    
    fn _addi(&mut self, mut d: Uint) {
        self.grow_by(1);
        
        for elem in self.data.iter_mut() {
            let (n, c) = elem.overflowing_add(d);
            (*elem, d) = (n, c as Uint);
        }
        
        self.shrink();
    }
}

impl Clone for Bignum {
    fn clone(&self) -> Bignum {
        Bignum {
            data: self.data.clone(),
            sign: self.sign
        }
    }
    
    fn clone_from(&mut self, oth: &Bignum) {
        self.sign = oth.sign;
        self.data.resize(oth.data.len(), oth.pad());
        self.data.clone_from_slice(&oth.data);
    }
}

impl Debug for Bignum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

impl BnIter<'_> {
    fn from(bn: &Bignum, len: usize) -> BnIter {
        BnIter {
            iter: bn.data.iter(),
            cnt: 0,
            max: len,
            pad: bn.pad()
        }
    }
}

impl Iterator for BnIter<'_> {
    type Item = Uint;
    
    fn next(&mut self) -> Option<<BnIter as Iterator>::Item> {
        if self.cnt == self.max {
            None
        } else {
            self.cnt += 1;
            
            match self.iter.next() {
                Some(elem) => Some(*elem),
                None => Some(self.pad)
            }
        }
    }
}
