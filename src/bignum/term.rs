use super::{Uint, Bignum, BnIter, BITS};

impl Bignum {
    pub fn inc(&mut self) {
        self.grow_by(1);
        
        for elem in self.data.iter_mut() {
            (*elem, _) = elem.overflowing_add(1);
            
            if *elem != 0 {
                break;
            }
        }
        
        self.sign = self.data[self.data.len() - 1] >> (BITS - 1) != 0;
        self.shrink();
    }
    
    pub fn add(&mut self, oth: &Bignum) {
        self.grow_to(oth.data.len());
        self.grow_by(1);
        let mut c = false;
        let len = self.data.len();
        
        for (s, o) in self.data.iter_mut().zip(BnIter::from(oth, len)) {
            let (s1, c1) = s.overflowing_add(o);
            let (s2, c2) = s1.overflowing_add(c as Uint);
            (*s, c) = (s2, c1 || c2); 
        }
        
        self.sign = self.data[self.data.len() - 1] >> (BITS - 1) != 0;
        self.shrink();
    }
    
    #[allow(unused)]
    pub fn dec(&mut self) {
        self.grow_by(1);
        
        for elem in self.data.iter_mut() {
            (*elem, _) = elem.overflowing_sub(1);
            
            if *elem != !0 {
                break;
            }
        }
        
        self.sign = self.data[self.data.len() - 1] >> (BITS - 1) != 0;
        self.shrink();
    }
    
    pub fn sub(&mut self, oth: &Bignum) {
        self.grow_to(oth.data.len());
        self.grow_by(1);
        let mut c = true;
        let len = self.data.len();
        
        for (s, o) in self.data.iter_mut().zip(BnIter::from(oth, len)) {
            let (s1, c1) = s.overflowing_add(o ^ !0);
            let (s2, c2) = s1.overflowing_add(c as Uint);
            (*s, c) = (s2, c1 || c2); 
        }
        
        self.sign = self.data[self.data.len() - 1] >> (BITS - 1) != 0;
        self.shrink();
    }
    
    pub fn neg(&mut self) {
        if !self.is_zero() {
            self.data.push(self.pad());
            self.sign ^= true;
            
            let mut c = true;
            
            for elem in self.data.iter_mut() {
                (*elem, c) = (*elem ^ !0).overflowing_add(c as Uint);
            }
            
            self.shrink();
        }
    }
    
    fn _extra(&self) -> Uint {
        // 00 => 01
        // ff => 00
        (self.pad() & !1) ^ 1
    }
}
