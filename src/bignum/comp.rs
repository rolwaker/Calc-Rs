use std::cmp::Ordering;

use super::Bignum;

impl Bignum {
    pub fn is_zero(&self) -> bool {
        !self.sign && self.data == [0, 0, 0, 0]
    }
    
    pub fn is_neg(&self) -> bool {
        self.sign
    }
    
    pub fn is_int(&self) -> bool {
        self.data[..4] == [0, 0, 0, 0]
    }
    
    pub fn is_even(&self) -> bool {
        self.is_int() && self.data.get(4).unwrap_or(&1) % 2 == 0
    }
}

impl PartialEq for Bignum {
    fn eq(&self, oth: &Bignum) -> bool {
        if self.data.len() != oth.data.len() || self.sign != oth.sign {
            false
        } else {
            for (s, o) in self.data.iter().zip(oth.data.iter()) {
                if s != o {
                    return false;
                }
            }
            
            true
        }
    }
    
    fn ne(&self, oth: &Bignum) -> bool {
        !self.eq(oth)
    }
}

impl PartialOrd for Bignum {
    fn partial_cmp(&self, oth: &Bignum) -> Option<Ordering> {
        Some(if self.sign != oth.sign {
            if self.sign {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            let mut res = Ordering::Equal;
            
            if self.data.len() > oth.data.len() {
                res = Ordering::Greater;
            } else if self.data.len() < oth.data.len() {
                res = Ordering::Less;
            } else {
                for (s, o) in self.data.iter().zip(oth.data.iter()).rev() {
                    // if signs match per-element comparison will work the same.
                    // 0xff > 0xfe no matter if it's signed or unsigned:
                    // sint: 255 > 254
                    // uint: -1 > -2
                    if s > o {
                        return Some(Ordering::Greater);
                    } else if s < o {
                        return Some(Ordering::Less);
                    }
                }
            }
            
            if self.sign {
                res.reverse()
            } else {
                res
            }
        })
    }
    
    fn gt(&self, oth: &Bignum) -> bool {
        self.partial_cmp(oth).unwrap() > Ordering::Equal
    }
    
    fn ge(&self, oth: &Bignum) -> bool {
        self.partial_cmp(oth).unwrap() >= Ordering::Equal
    }
    
    fn lt(&self, oth: &Bignum) -> bool {
        self.partial_cmp(oth).unwrap() < Ordering::Equal
    }
    
    fn le(&self, oth: &Bignum) -> bool {
        self.partial_cmp(oth).unwrap() <= Ordering::Equal
    }
}
