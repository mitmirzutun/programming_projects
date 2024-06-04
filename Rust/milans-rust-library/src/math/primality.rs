macro_rules! pow_mod {
    ($type1:ty,$type2:ty) => {
        impl PowMod for $type1 {
            fn mul_mod(self, other: $type1, modulus: $type1) -> $type1 {
                (self as $type2*other as $type2%modulus as $type2) as $type1
            }
            fn pow_mod(self, exponent: u128, modulus: $type1) -> $type1 {
                let mut base=self;
                let mut result=1;
                for bit in 0..128 {
                    if exponent&(1<<bit)>0 {result=result.mul_mod(base,modulus)}
                    base=base.mul_mod(base,modulus);
                }
                result
            }
        }
    };
    ($($type1:ty,$type2:ty);+) => {
        $(pow_mod!($type1,$type2);)+
    };
}
pub trait PowMod {
    fn mul_mod(self, other: Self, modulus: Self) -> Self;
    fn pow_mod(self, exponent: u128, modulus: Self) -> Self;
}
pow_mod!(u8,u16;i8,i16;u16,u32;i16,i32;u32,u64;i32,i64;u64,u128;i64,i128;usize,u128;isize,i128);
impl PowMod for u128 {
    fn mul_mod(self, other: Self, modulus: Self) -> Self {
        use num::ToPrimitive;
        use num_bigint::ToBigInt;
        ((self.to_bigint().unwrap() * other.to_bigint().unwrap()) % modulus.to_bigint().unwrap())
            .to_u128()
            .unwrap()
    }
    fn pow_mod(self, exponent: u128, modulus: u128) -> u128 {
        let mut base = self;
        let mut result = 1;
        for bit in 0..128 {
            if exponent & (1 << bit) > 0 {
                result = result.mul_mod(base, modulus)
            }
            base = base.mul_mod(base, modulus);
        }
        result
    }
}
impl PowMod for i128 {
    fn mul_mod(self, other: Self, modulus: Self) -> Self {
        use num::ToPrimitive;
        use num_bigint::ToBigInt;
        ((self.to_bigint().unwrap() * other.to_bigint().unwrap()) % modulus.to_bigint().unwrap())
            .to_i128()
            .unwrap()
    }
    fn pow_mod(self, exponent: u128, modulus: i128) -> i128 {
        let mut base = self;
        let mut result = 1;
        for bit in 0..128 {
            if exponent & (1 << bit) > 0 {
                result = result.mul_mod(base, modulus)
            }
            base = base.mul_mod(base, modulus);
        }
        result
    }
}
impl<T> PowMod for std::rc::Rc<T>
where
    T: PowMod + Copy,
{
    fn mul_mod(self, other: Self, modulus: Self) -> Self {
        std::rc::Rc::new(<T as PowMod>::mul_mod(*self, *other, *modulus))
    }
    fn pow_mod(self, exponent: u128, modulus: Self) -> Self {
        std::rc::Rc::new(<T as PowMod>::pow_mod(*self, exponent, *modulus))
    }
}
impl<T> PowMod for std::sync::Arc<T>
where
    T: PowMod + Copy,
{
    fn mul_mod(self, other: Self, modulus: Self) -> Self {
        std::sync::Arc::new(<T as PowMod>::mul_mod(*self, *other, *modulus))
    }
    fn pow_mod(self, exponent: u128, modulus: Self) -> Self {
        std::sync::Arc::new(<T as PowMod>::pow_mod(*self, exponent, *modulus))
    }
}
macro_rules! miller_rabin_is_witness {
    ($prime:expr,$base:expr,$exponent:expr,$squares:expr) => {{
        let prime = $prime;
        let base = $base;
        let exponent = $exponent as u128;
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
                if result < 2 {
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
    fn miller_rabin_test(self) -> bool;
    fn miller_rabin_test_iter(self, iterations: usize) -> bool;
}
impl MillerRabinTest for u8 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for i8 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for u16 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if miller_rabin_is_witness!(prime, 7, exponent, squares) {
            return false;
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if miller_rabin_is_witness!(prime, 7, exponent, squares) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for i16 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if miller_rabin_is_witness!(prime, 7, exponent, squares) {
            return false;
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if miller_rabin_is_witness!(prime, 7, exponent, squares) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for u32 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if miller_rabin_is_witness!(prime, 7, exponent, squares)
            || miller_rabin_is_witness!(prime, 61, exponent, squares)
        {
            return false;
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if miller_rabin_is_witness!(prime, 7, exponent, squares) {
            return false;
        }
        if iterations == 2 {
            return true;
        }
        if miller_rabin_is_witness!(prime, 61, exponent, squares) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for i32 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if miller_rabin_is_witness!(prime, 7, exponent, squares)
            || miller_rabin_is_witness!(prime, 61, exponent, squares)
        {
            return false;
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if miller_rabin_is_witness!(prime, 7, exponent, squares) {
            return false;
        }
        if iterations == 2 {
            return true;
        }
        if miller_rabin_is_witness!(prime, 61, exponent, squares) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for u64 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as u64 {
            let prime = prime as u32;
            if miller_rabin_is_witness!(prime, 7, exponent, squares)
                || miller_rabin_is_witness!(prime, 61, exponent, squares)
            {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2) {
            if miller_rabin_is_witness!(prime, a, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as u64 {
            let prime = prime as u32;
            if miller_rabin_is_witness!(prime, 7, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_is_witness!(prime, 61, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2).take(iterations - 1) {
            if miller_rabin_is_witness!(prime, a, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl MillerRabinTest for i64 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as i64 {
            let prime = prime as u32;
            if miller_rabin_is_witness!(prime, 7, exponent, squares)
                || miller_rabin_is_witness!(prime, 61, exponent, squares)
            {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2) {
            if miller_rabin_is_witness!(prime, a, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as i64 {
            if miller_rabin_is_witness!(prime, 7, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_is_witness!(prime, 61, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2) {
            if miller_rabin_is_witness!(prime, a, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl MillerRabinTest for usize {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as usize {
            let prime = prime as u32;
            if miller_rabin_is_witness!(prime, 7, exponent, squares)
                || miller_rabin_is_witness!(prime, 61, exponent, squares)
            {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2) {
            if miller_rabin_is_witness!(prime, a, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as usize {
            let prime = prime as u32;
            if miller_rabin_is_witness!(prime, 7, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_is_witness!(prime, 61, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2).take(iterations - 1) {
            if miller_rabin_is_witness!(prime, a, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl MillerRabinTest for isize {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as isize {
            let prime = prime as u32;
            if miller_rabin_is_witness!(prime, 7, exponent, squares)
                || miller_rabin_is_witness!(prime, 61, exponent, squares)
            {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2) {
            if miller_rabin_is_witness!(prime, a, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as isize {
            if miller_rabin_is_witness!(prime, 7, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_is_witness!(prime, 61, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2) {
            if miller_rabin_is_witness!(prime, a, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl MillerRabinTest for u128 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as u128 {
            let prime = prime as u32;
            if miller_rabin_is_witness!(prime, 7, exponent, squares)
                || miller_rabin_is_witness!(prime, 61, exponent, squares)
            {
                return false;
            }
            return true;
        }
        if prime <= u64::MAX as u128 {
            let prime = prime as u64;
            for a in (3..=43).step_by(2) {
                if miller_rabin_is_witness!(prime, a, exponent, squares) {
                    return false;
                }
            }
            return true;
        }
        for a in 3..=2 * 128 * 128 {
            if miller_rabin_is_witness!(prime, a, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as u128 {
            let prime = prime as u32;
            if miller_rabin_is_witness!(prime, 7, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_is_witness!(prime, 61, exponent, squares) {
                return false;
            }
            return false;
        }
        if prime <= u64::MAX as u128 {
            let prime = prime as u64;
            for a in (3..=43).step_by(2).take(iterations - 1) {
                if miller_rabin_is_witness!(prime, a, exponent, squares) {
                    return false;
                }
            }
            return true;
        }
        for a in 3..=2 * 128 * 128 {
            if miller_rabin_is_witness!(prime, a, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl MillerRabinTest for i128 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as i128 {
            let prime = prime as u32;
            if miller_rabin_is_witness!(prime, 7, exponent, squares)
                || miller_rabin_is_witness!(prime, 61, exponent, squares)
            {
                return false;
            }
            return true;
        }
        if prime <= u64::MAX as i128 {
            let prime = prime as u64;
            for a in (3..=43).step_by(2) {
                if miller_rabin_is_witness!(prime, a, exponent, squares) {
                    return false;
                }
            }
            return true;
        }
        for a in 3..=2 * 128 * 128 {
            if miller_rabin_is_witness!(prime, a, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        let prime = self;
        if self < 2 {
            return false;
        }
        if self % 6 != 1 && self % 6 != 3 {
            return prime < 4;
        }
        if iterations == 0 {
            return true;
        }
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as i128 {
            let prime = prime as u32;
            if miller_rabin_is_witness!(prime, 7, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_is_witness!(prime, 61, exponent, squares) {
                return false;
            }
            return false;
        }
        if prime <= u64::MAX as i128 {
            let prime = prime as u64;
            for a in (3..=43).step_by(2).take(iterations - 1) {
                if miller_rabin_is_witness!(prime, a, exponent, squares) {
                    return false;
                }
            }
            return true;
        }
        for a in 3..=2 * 128 * 128 {
            if miller_rabin_is_witness!(prime, a, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl<T> MillerRabinTest for &T
where
    T: MillerRabinTest + Clone,
{
    fn miller_rabin_test(self) -> bool {
        <T as MillerRabinTest>::miller_rabin_test(self.clone())
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        <T as MillerRabinTest>::miller_rabin_test_iter(self.clone(), iterations)
    }
}
impl<T> MillerRabinTest for std::rc::Rc<T>
where
    T: MillerRabinTest + Clone,
{
    fn miller_rabin_test(self) -> bool {
        <T as MillerRabinTest>::miller_rabin_test((*self).clone())
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        <T as MillerRabinTest>::miller_rabin_test_iter((*self).clone(), iterations)
    }
}
impl<T> MillerRabinTest for std::sync::Arc<T>
where
    T: MillerRabinTest + Clone,
{
    fn miller_rabin_test(self) -> bool {
        <T as MillerRabinTest>::miller_rabin_test((*self).clone())
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        <T as MillerRabinTest>::miller_rabin_test_iter((*self).clone(), iterations)
    }
}
