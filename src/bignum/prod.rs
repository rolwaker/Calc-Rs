use std::mem::swap;

use super::{Uint, Bignum, BITS};

enum Mode {
    Div,
    Rem,
}

impl Bignum {
    pub fn mul(&mut self, oth: &Bignum) {
        let mut n = Bignum::zero();
        let sign = self.sign ^ oth.sign;
        let mut bit = 0;
        let mut c = oth.sign;
        let mut val = 0;
        
        if self.sign {
            self.neg();
        }
        
        swap(self, &mut n);
        
        while bit / BITS < oth.data.len() {
            // negate rhs if needed as we go to avoid copy.
            if bit % BITS == 0 {
                val = oth.data[bit / BITS] ^ oth.pad();
                (val, c) = val.overflowing_add(c as Uint);
            }
            
            if (val & (1 << (bit % BITS))) != 0 {
                self.add(&n);
            }
            
            n.shl(1);
            bit += 1;
        }
        
        if c {
            self.add(&n);
        }
        
        if sign {
            self.neg();
        }
        
        self.shr(4 * BITS);
        
        self.shrink();
    }
    
    pub fn div(&mut self, oth: &Bignum) {
        self.shl(4 * BITS);
        self._div(oth, Mode::Div);
    }
    
    pub fn rem(&mut self, oth: &Bignum) {
        self._div(oth, Mode::Rem);
    }
    
    fn _div(&mut self, oth: &Bignum, mode: Mode) {
        let mut n = oth.clone();
        let mut q = Bignum::zero();
        let mut sign = self.sign;
        let mut bit = 0;
        
        // duplication based on sign of rhs to avoid unnecessary copy.
        if oth.sign {
            if !self.sign {
                self.neg();
            }
            
            while *self <= n {//println!("1 {:?} {:?} {:?}", self, n, oth);
                n.shl(1);
                bit += 1;
            }
            
            q.grow_to(bit / BITS + (bit % BITS != 0) as usize);
            
            while *self <= *oth {//println!("2 {:?} {:?} {:?}", self, n, oth);
                while *self < n {//println!("3 {:?} {:?} {:?}", self, n, oth);
                    n._div2();
                    bit -= 1;
                }
                
                self.sub(&n);
                q.data[bit / BITS] |= 1 << (bit % BITS);
            }
            
            q.shrink();
            
            match mode {
                Mode::Div => {
                    sign ^= true;
                    swap(self, &mut q);
                },
                Mode::Rem => {
                    if !sign {
                        self.neg();
                    }
                }
            }
        } else {
            if sign {
                self.neg();
            }
            
            while *self >= n {//println!("4 {:?} {:?} {:?}", self, n, oth);
                n.shl(1);
                bit += 1;
            }
            
            q.grow_to(bit / BITS + (bit % BITS != 0) as usize);
            
            while *self >= *oth {//println!("5 {:?} {:?} {:?}", self, n, oth);
                while *self < n {//println!("6 {:?} {:?} {:?}", self, n, oth);
                    n.shr(1);
                    bit -= 1;
                }
                
                self.sub(&n);
                q.data[bit / BITS] |= 1 << (bit % BITS);
            }
            
            q.shrink();
            
            match mode {
                Mode::Div => swap(self, &mut q),
                Mode::Rem => {}
            }
        }
        
        if sign {
            self.neg();
        }
    }
    
    fn _div2(&mut self) {
        let is_odd = self.data[0] % 2 != 0;
        
        self.shr(1);
        
        if self.sign && is_odd {
            self.inc();
        }
    }
}
