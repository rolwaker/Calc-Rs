use super::{Bignum, BITS};

impl Bignum {
    pub fn shl(&mut self, mut sh: usize) {
        let mut i;
        
        if sh >= BITS {
            let d = sh / BITS;
            sh %= BITS;
            self.grow_by(d);
            i = self.data.len() - 1;
            
            while i >= d {
                self.data[i] = self.data[i - d];
                i -= 1;
            }
            
            loop {
                self.data[i] = 0;
                
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            }
        }
        
        if sh != 0 {
            self.grow_by(1);
            i = self.data.len() - 1;
            
            while i > 0 {
                self.data[i] = (self.data[i] << sh) | (self.data[i - 1] >> (BITS - sh));
                i -= 1;
            }
            
            self.data[i] <<= sh;
        }
        
        self.shrink();
    }
    
    pub fn shr(&mut self, mut sh: usize) {
        let mut i;
        
        if sh >= BITS {
            let d = sh / BITS;
            sh %= BITS;
            
            i = 0;
            
            while i + d < self.data.len() {
                self.data[i] = self.data[i + d];
                i += 1;
            }
            
            while i < self.data.len() {
                self.data[i] = self.pad();
                i += 1;
            }
        }
        
        if sh != 0 {
            i = 0;
            
            while i < self.data.len() - 1 {
                self.data[i] = (self.data[i] >> sh) | (self.data[i + 1] << (BITS - sh));
                i += 1;
            }
            
            self.data[i] >>= sh;
            
            if self.sign {
                self.data[i] |= ((1 << sh) - 1) << (BITS - sh);
            }
        }
        
        self.shrink();
    }
}
