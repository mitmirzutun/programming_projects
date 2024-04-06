macro_rules! pow_mod {
    ($type1:ty,$type2:ty) => {
        impl PowMod for $type1 {
            fn mul_mod(self, other: $type1,modulus:$type1) -> $type1 {
                let result=(self as $type2*other as $type2%modulus as $type2);
                $crate::trace!("<{} as PowMod>::mul_mod({self}, {other}, {modulus})={result}",stringify!($type1)).unwrap();
                result as $type1
            }
            fn pow_mod(self,exponent:u128,modulus:$type1) -> $type1 {
                let mut result=1;
                let mut base=self;
                for bit in 0..128 {
                    if exponent&(1<<bit)>0 {
                        result=base.mul_mod(result,modulus);
                    }
                    base=base.mul_mod(base,modulus);
                }
                result
            }
        }
    };
    ($($type1:ty,$type2:ty),+) => {
        $(pow_mod!($type1,$type2);)+
    };
}
pub trait PowMod {
    fn mul_mod(self, other: Self, modulus: Self) -> Self;
    fn pow_mod(self, exponent: u128, modulus: Self) -> Self;
}
pow_mod!(
    u8, u16, i8, i16, u16, u32, i16, i32, u32, u64, i32, i64, u64, u128, i64, i128, usize, u128,
    isize, i128
);
impl PowMod for u128 {
    fn mul_mod(self, other: u128, modulus: u128) -> u128 {
        use num::ToPrimitive;
        use num_bigint::ToBigInt;
        let result =
            self.to_bigint().unwrap() * other.to_bigint().unwrap() % modulus.to_bigint().unwrap();
        crate::trace!("<u128 as PowMod>::mul_mod({self}, {other}, {modulus})={result}").unwrap();
        result.to_u128().unwrap()
    }
    fn pow_mod(self, exponent: u128, modulus: u128) -> u128 {
        let mut result = 1;
        let mut base = self;
        for bit in 0..128 {
            if exponent & (1 << bit) > 0 {
                result = base.mul_mod(result, modulus);
            }
            base = base.mul_mod(base, modulus);
        }
        result
    }
}
impl PowMod for i128 {
    fn mul_mod(self, other: i128, modulus: i128) -> i128 {
        use num::ToPrimitive;
        use num_bigint::ToBigInt;
        let result =
            self.to_bigint().unwrap() * other.to_bigint().unwrap() % modulus.to_bigint().unwrap();
        crate::trace!("<u128 as PowMod>::mul_mod({self}, {other}, {modulus})={result}").unwrap();
        result.to_i128().unwrap()
    }
    fn pow_mod(self, exponent: u128, modulus: i128) -> i128 {
        let mut result = 1;
        let mut base = self;
        for bit in 0..128 {
            if exponent & (1 << bit) > 0 {
                result = base.mul_mod(result, modulus);
            }
            base = base.mul_mod(base, modulus);
        }
        result
    }
}
pub trait PowModRef {
    fn mul_mod(&self, other: Self, modulus: Self) -> Self;
    fn pow_mod(&self, exponent: u128, modulus: Self) -> Self;
}
impl<T> PowModRef for T
where
    T: PowMod + Copy,
{
    fn mul_mod(&self, other: Self, modulus: Self) -> Self {
        PowMod::mul_mod(*self, other, modulus)
    }
    fn pow_mod(&self, exponent: u128, modulus: Self) -> Self {
        PowMod::pow_mod(*self, exponent, modulus)
    }
}
macro_rules! miller_rabin_test {
    ($base:expr,$prime:expr) => {{
        let base = $base;
        let prime = $prime;
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        miller_rabin_test!(base, prime, exponent, squares)
    }};
    ($base:expr,$prime:expr,$exponent:expr,$squares:expr) => {{
        let base = $base;
        let prime = $prime;
        let exponent = $exponent;
        let squares = $squares;
        let mut result = base.pow_mod(exponent, prime);
        if result == 0 {
            true
        } else if result == 1 || result == prime - 1 {
            false
        } else {
            let mut is_witness = true;
            for _ in 0..squares {
                result = result.mul_mod(result, prime);
                if result <= 1 {
                    break;
                }
                if result == prime - 1 {
                    is_witness = false;
                    break;
                }
            }
            is_witness
        }
    }};
}
pub trait MillerRabinTest {
    fn test(self) -> bool;
    fn test_iter(self, iterations: usize) -> bool;
}
impl<T> MillerRabinTest for &T
where
    T: MillerRabinTest + Clone,
{
    fn test(self) -> bool {
        self.clone().test()
    }
    fn test_iter(self, iterations: usize) -> bool {
        self.clone().test_iter(iterations)
    }
}
impl<T> MillerRabinTest for std::rc::Rc<T>
where
    T: MillerRabinTest + Clone,
{
    fn test(self) -> bool {
        self.as_ref().clone().test()
    }
    fn test_iter(self, iterations: usize) -> bool {
        self.as_ref().clone().test_iter(iterations)
    }
}
impl MillerRabinTest for u8 {
    fn test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if miller_rabin_test!(2, prime) {
            return false;
        }
        true
    }
    fn test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        if miller_rabin_test!(2, prime) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for i8 {
    fn test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if miller_rabin_test!(2, prime) {
            return false;
        }
        true
    }
    fn test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        if miller_rabin_test!(2, prime) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for u16 {
    fn test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if miller_rabin_test!(7, prime, exponent, squares) {
            return false;
        }
        true
    }
    fn test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if miller_rabin_test!(7, prime, exponent, squares) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for i16 {
    fn test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if miller_rabin_test!(7, prime, exponent, squares) {
            return false;
        }
        true
    }
    fn test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if miller_rabin_test!(7, prime, exponent, squares) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for u32 {
    fn test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if miller_rabin_test!(7, prime, exponent, squares) {
            return false;
        }
        if miller_rabin_test!(61, prime, exponent, squares) {
            return false;
        }
        true
    }
    fn test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if miller_rabin_test!(7, prime, exponent, squares) {
            return false;
        }
        if iterations == 2 {
            return true;
        }
        if miller_rabin_test!(61, prime, exponent, squares) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for i32 {
    fn test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if miller_rabin_test!(7, prime, exponent, squares) {
            return false;
        }
        if miller_rabin_test!(61, prime, exponent, squares) {
            return false;
        }
        true
    }
    fn test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if miller_rabin_test!(7, prime, exponent, squares) {
            return false;
        }
        if iterations == 2 {
            return true;
        }
        if miller_rabin_test!(61, prime, exponent, squares) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for u64 {
    fn test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as u64 {
            let prime = prime as u32;
            if miller_rabin_test!(7, prime, exponent, squares) {
                return false;
            }
            if miller_rabin_test!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2) {
            if miller_rabin_test!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as u64 {
            let prime = prime as u32;
            if miller_rabin_test!(7, prime, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return false;
            }
            if miller_rabin_test!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2).take(iterations - 1) {
            if miller_rabin_test!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl MillerRabinTest for i64 {
    fn test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as i64 {
            let prime = prime as u32;
            if miller_rabin_test!(7, prime, exponent, squares) {
                return false;
            }
            if miller_rabin_test!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2) {
            if miller_rabin_test!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as i64 {
            let prime = prime as u32;
            if miller_rabin_test!(7, prime, exponent, squares) {
                return false;
            }
            if miller_rabin_test!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2) {
            if miller_rabin_test!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl MillerRabinTest for u128 {
    fn test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as u128 {
            let prime = prime as u32;
            if miller_rabin_test!(7, prime, exponent, squares) {
                return false;
            }
            if miller_rabin_test!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        if prime <= u64::MAX as u128 {
            let prime = prime as u64;
            for a in (3..=43).step_by(2) {
                if miller_rabin_test!(a, prime, exponent, squares) {
                    return false;
                }
            }
            return true;
        }
        for a in 3..=((prime.ilog2() + 1) as u128 * (prime.ilog2() + 1) as u128) * 2 {
            if miller_rabin_test!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as u128 {
            let prime = prime as u32;
            if miller_rabin_test!(7, prime, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return false;
            }
            if miller_rabin_test!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        if prime <= u64::MAX as u128 {
            let prime = prime as u64;
            for a in (3..=43).step_by(2).take(iterations - 1) {
                if miller_rabin_test!(a, prime, exponent, squares) {
                    return false;
                }
            }
            return true;
        }
        for a in (3..=((prime.ilog2() + 1) as u128 * (prime.ilog2() + 1) as u128) * 2)
            .take(iterations - 1)
        {
            if miller_rabin_test!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl MillerRabinTest for i128 {
    fn test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as i128 {
            let prime = prime as u32;
            if miller_rabin_test!(7, prime, exponent, squares) {
                return false;
            }
            if miller_rabin_test!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        if prime <= u64::MAX as i128 {
            let prime = prime as u64;
            for a in (3..=43).step_by(2) {
                if miller_rabin_test!(a, prime, exponent, squares) {
                    return false;
                }
            }
            return true;
        }
        for a in 3..=((prime.ilog2() + 1) as i128 * (prime.ilog2() + 1) as i128) * 2 {
            if miller_rabin_test!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as i128 {
            let prime = prime as u32;
            if miller_rabin_test!(7, prime, exponent, squares) {
                return false;
            }
            if miller_rabin_test!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        if prime <= u64::MAX as i128 {
            let prime = prime as u64;
            for a in (3..=43).step_by(2) {
                if miller_rabin_test!(a, prime, exponent, squares) {
                    return false;
                }
            }
            return true;
        }
        for a in (3..=((prime.ilog2() + 1) as i128 * (prime.ilog2() + 1) as i128) * 2)
            .take(iterations - 1)
        {
            if miller_rabin_test!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl MillerRabinTest for usize {
    fn test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as usize {
            let prime = prime as u32;
            if miller_rabin_test!(7, prime, exponent, squares) {
                return false;
            }
            if miller_rabin_test!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2) {
            if miller_rabin_test!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as usize {
            let prime = prime as u32;
            if miller_rabin_test!(7, prime, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return false;
            }
            if miller_rabin_test!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2).take(iterations - 1) {
            if miller_rabin_test!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl MillerRabinTest for isize {
    fn test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as isize {
            let prime = prime as u32;
            if miller_rabin_test!(7, prime, exponent, squares) {
                return false;
            }
            if miller_rabin_test!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2) {
            if miller_rabin_test!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        if miller_rabin_test!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as isize {
            let prime = prime as u32;
            if miller_rabin_test!(7, prime, exponent, squares) {
                return false;
            }
            if miller_rabin_test!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2) {
            if miller_rabin_test!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl MillerRabinTest for num_bigint::BigUint {
    fn test(self) -> bool {
        use num::ToPrimitive;
        use num_bigint::{ToBigInt, ToBigUint};
        let prime = self;
        if prime < 2.to_biguint().unwrap() {
            return false;
        }
        let rem = prime.clone() % 6.to_biguint().unwrap();
        if rem != 1.to_biguint().unwrap() && rem != 5.to_biguint().unwrap() {
            return prime < 4.to_biguint().unwrap();
        }
        if prime.clone() % 5.to_biguint().unwrap() != 0.to_biguint().unwrap() {
            return prime == 5.to_biguint().unwrap();
        }
        let exponent: num_bigint::BigUint = prime.clone() >> 1;
        let squares = exponent.trailing_zeros().unwrap();
        let exponent = exponent >> squares;
        let mut result = 2.to_biguint().unwrap().modpow(&exponent, &prime);
        if result != 1.to_biguint().unwrap() && result != prime.clone() - 1.to_biguint().unwrap() {
            let mut is_witness = true;
            for _ in 0..squares {
                result = result.clone() * result.clone() % prime.clone();
                if result < 2.to_biguint().unwrap() {
                    return false;
                }
                if result == prime.clone() - 1.to_biguint().unwrap() {
                    is_witness = false;
                    break;
                }
            }
            if is_witness {
                return false;
            }
        }
        if prime < 2047.to_biguint().unwrap() {
            return true;
        }
        if prime <= u32::MAX.to_biguint().unwrap() {
            let prime = prime.to_u32().unwrap();
        }
        true
    }
    fn test_iter(self, iterations: usize) -> bool {
        true
    }
}
