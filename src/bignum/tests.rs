use super::Bignum;

impl Bignum {
    #[allow(unused)]
    fn lit(digits: &str) -> Bignum {
        Bignum::from(digits).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    
    #[test]
    fn sanity() {
        let mut a = Bignum::lit("23");
        let mut b = Bignum::lit("3");
        let c;
        
        b.sub(&a);
        assert_eq!(b, Bignum::lit("-20"));
        
        c = b.clone();
        b.sub(&c);
        assert_eq!(b, Bignum::lit("0"));
        b.add(&c);
        
        b.add(&a);
        assert_eq!(b, Bignum::lit("3"));
        
        a.sub(&b);
        assert_eq!(a, Bignum::lit("20"));
        
        a.add(&b);
        assert_eq!(a, Bignum::lit("23"));
        
        a.neg();
        a.add(&b);
        assert_eq!(a, Bignum::lit("-20"));
        
        b.shl(1);
        assert_eq!(b, Bignum::lit("6"));
        
        b.sub(&a);
        assert_eq!(b, Bignum::lit("26"));
        
        a.mul(&b);
        assert_eq!(a, Bignum::lit("-520"));
        
        b.neg();
        assert_eq!(b, Bignum::lit("-26"));
        
        a.mul(&b);
        assert_eq!(a, Bignum::lit("13520"));
    }
    
    #[test]
    fn sign() {
        let mut a = Bignum::lit("-1000");
        let b = Bignum::lit("1");
        let mut c = Bignum::zero();
        let d = Bignum::lit("1000");
        
        while a < d {
            c.clone_from(&a);
            c.neg();
            
            if c.sign {
                assert!(c < a);
            } else if c.is_zero() {
                assert_eq!(c, a);
            } else {
                assert!(c > a);
            }
            
            c.neg();
            assert_eq!(a, c);
            
            a.add(&b);
        }
    }
    
    #[test]
    fn ordering() {
        let mut a = Bignum::lit("-23357");
        let mut b = Bignum::lit("23");
        
        assert!(a < b);
        
        a.neg();
        assert!(a > b);
        
        let mut c = b.clone();
        
        b.add(&a);
        assert!(b > a);
        assert!(b > c && a > c);
        
        c.add(&a);
        assert!(c > a);
        assert!(b > a);
        assert!(b == c);
    }
    
    #[test]
    fn shift() {
        let mut a = Bignum::lit("-23");
        let mut b = Bignum::lit("-23");
        
        b.shl(1);
        assert_eq!(b, Bignum::lit("-46"));
        
        b.neg();
        b.add(&a);
        assert_eq!(b, Bignum::lit("23"));
        
        a.shl(2);
        a.sub(&b);
        assert_eq!(a, Bignum::lit("-115"));
    }
    
    #[test]
    fn scale() {
        let mut a = Bignum::lit("435");
        let mut b = Bignum::lit("1");
        let mut i = 0;
        
        while i < 10 {
            let mut c = a.clone();
            let mut d = a.clone();
            
            c.shl(i);
            d.mul(&b);
            
            assert_eq!(c, d);
            
            i += 1;
            b.shl(1);
        }
        
        a.shl(1);
        b.shr(10);
        i = 0;
        
        while i < 10 {
            let mut c = a.clone();
            let mut d = a.clone();
            
            c.shr(i);
            d.div(&b);
            
            assert_eq!(c, d);
            
            i += 1;
            b.shl(1);
        }
    }
}
