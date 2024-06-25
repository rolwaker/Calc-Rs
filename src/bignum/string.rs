use super::{Uint, Big, Bignum, BITS};

fn slice_zero(slice: &[Uint]) -> bool {
    for elem in slice.iter() {
        if *elem != 0 {
            return false;
        }
    }
    
    true
}

fn slice_inc(slice: &mut [Uint]) {
    for elem in slice.iter_mut() {
        (*elem, _) = elem.overflowing_add(1);
        
        if *elem != 0 {
            break;
        }
    }
}

fn slice_dec(slice: &mut [Uint]) {
    for elem in slice.iter_mut() {
        (*elem, _) = elem.overflowing_sub(1);
        if *elem != !0 {
            break;
        }
    }
}

fn slice_muli(slice: &mut [Uint], fact: Big) {
    let mut hi = 0;
    
    for elem in slice.iter_mut() {
        let n = (*elem as Big) * fact + hi;
        (*elem, hi) = (n as Uint, n >> BITS);
    }
}

fn slice_divi(slice: &mut [Uint], div: Big) -> Uint {
    let mut rem = 0;
    
    for elem in slice.iter_mut().rev() {
        let n = ((rem as Big) << BITS) | *elem as Big;
        (*elem, rem) = ((n / div) as Uint, (n % div) as Uint);
    }
    
    rem
}

// not general purpose, meant to divide out digits only.
fn slice_div(slice: &mut [Uint], quot: &[Uint]) -> u32 {
    let mut digit = 0;
    
    loop {
        for (s, q) in slice.iter().zip(quot.iter()).rev() {
            if s > q {
                break;
            } else if s < q {
                return digit;
            }
        }
        
        let mut c = true;
        
        for (s, q) in slice.iter_mut().zip(quot.iter()) {
            let (s1, c1) = s.overflowing_add(q ^ !0);
            let (s2, c2) = s1.overflowing_add(c as Uint);
            (*s, c) = (s2, c1 || c2); 
        }
        
        digit += 1;
    }
}

impl ToString for Bignum {
    // convert bignum to string with smallest number of decimal places
    // for a unique representation. (fixed-point to string is easy!)
    fn to_string(&self) -> String {
        let mut repr = String::new();
        let sign = self.sign;
        
        let mut bn = if self.sign {
            let mut n = self.clone();
            n.neg();
            n
        } else {
            self.clone()
        };
        
        let (frac, whole) = bn.data.split_at_mut(4);
        
        loop {
            repr.push(char::from_digit(slice_divi(whole, 10) as u32, 10).unwrap());
            
            if slice_zero(whole) {
                break;
            }
        }
        
        if sign {
            repr.push('-');
        }
        
        repr = repr.chars().rev().collect();
        
        if !slice_zero(frac) {
            let mut prev = [0, 0, 0, 0, 0];
            let mut curr = [0, 0, 0, 0, 0];
            let mut next = [0, 0, 0, 0, 0];
            let quot = [0, 0, 0, 0, 1];
            let mut zeroes = 0;
            let mut i = 0;
            let mut pc;
            let mut cc;
            let mut nc;
            
            while i < 4 {
                prev[i] = frac[i];
                curr[i] = frac[i];
                next[i] = frac[i];
                i += 1;
            }
            
            slice_dec(&mut prev[..4]);
            slice_inc(&mut next[..4]);
            
            repr.push('.');
            
            loop {
                slice_muli(&mut prev, 10);
                slice_muli(&mut curr, 10);
                slice_muli(&mut next, 10);
                
                pc = slice_div(&mut prev, &quot);
                cc = slice_div(&mut curr, &quot);
                nc = slice_div(&mut next, &quot);
                
                if cc == 0 {
                    zeroes += 1;
                } else {
                    while zeroes != 0 {
                        repr.push('0');
                        zeroes -= 1;
                    }
                    
                    repr.push(char::from_digit(cc, 10).unwrap());
                }
                
                //println!("{}{}{}", pc, cc, nc);
                
                if pc != cc && cc != nc || slice_zero(&curr) {
                    break;
                }
            }
        }
        
        repr
    }
}
