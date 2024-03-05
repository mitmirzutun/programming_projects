macro_rules! pow_mod {
    ($type1:ty,$type2:ty) => {
        impl PowMod for $type1 {
            fn mul_mod(self, other: Self, modulus: Self) -> Self {
                let result = (self as $type2 * other as $type2 % modulus as $type2) as $type1;
                $crate::trace!(
                    "<{} as PowMod>::mul_mod({}, {}, {})={}",
                    stringify!($type1),
                    self,
                    other,
                    modulus,
                    result
                );
                result
            }
            fn pow_mod(self, exponent: u128, modulus: Self) -> Self {
                $crate::debug!(
                    "<{} as PowMod>::pow_mod({}, {}, {})",
                    stringify!($type1),
                    self,
                    exponent,
                    modulus
                );
                let mut base = self;
                let mut result = 1;
                for bit in 0..128 - exponent.leading_ones() {
                    if (1 << bit) & exponent > 0 {
                        result = result.mul_mod(base, modulus)
                    }
                    base = base.mul_mod(base, modulus)
                }
                $crate::debug!(
                    "<{} as PowMod>::pow_mod({}, {}, {})={}",
                    stringify!($type1),
                    self,
                    exponent,
                    modulus,
                    result
                );
                result
            }
        }
    };
}
pub trait PowMod {
    fn mul_mod(self, other: Self, modulus: Self) -> Self;
    fn pow_mod(self, exponent: u128, modulus: Self) -> Self;
}
pow_mod!(u8, u16);
pow_mod!(i8, i16);
pow_mod!(u16, u32);
pow_mod!(i16, i32);
pow_mod!(u32, u64);
pow_mod!(i32, i64);
pow_mod!(u64, u128);
pow_mod!(i64, i128);
impl PowMod for u128 {
    fn mul_mod(self, other: Self, modulus: Self) -> Self {
        use num::ToPrimitive;
        use num_bigint::ToBigUint;
        let result = self.to_biguint().unwrap() * other.to_biguint().unwrap()
            % modulus.to_biguint().unwrap();
        result.to_u128().unwrap()
    }
    fn pow_mod(self, exponent: u128, modulus: Self) -> Self {
        let mut base = self;
        let mut result = 1;
        for bit in 0..128 - exponent.leading_ones() {
            if (1 << bit) & exponent > 0 {
                result = result.mul_mod(base, modulus)
            }
            base = base.mul_mod(base, modulus)
        }
        result
    }
}
impl PowMod for i128 {
    fn mul_mod(self, other: Self, modulus: Self) -> Self {
        use num::ToPrimitive;
        use num_bigint::ToBigInt;
        let result =
            self.to_bigint().unwrap() * other.to_bigint().unwrap() % modulus.to_bigint().unwrap();
        result.to_i128().unwrap()
    }
    fn pow_mod(self, exponent: u128, modulus: Self) -> Self {
        let mut base = self;
        let mut result = 1;
        for bit in 0..128 - exponent.leading_ones() {
            if (1 << bit) & exponent > 0 {
                result = result.mul_mod(base, modulus)
            }
            base = base.mul_mod(base, modulus)
        }
        result
    }
}
pow_mod!(usize, u128);
pow_mod!(isize, i128);
impl<T> PowMod for std::rc::Rc<T>
where
    T: PowMod + Copy,
{
    fn mul_mod(self, other: Self, modulus: Self) -> Self {
        std::rc::Rc::new(PowMod::mul_mod(*self, *other, *modulus))
    }
    fn pow_mod(self, exponent: u128, modulus: Self) -> Self {
        std::rc::Rc::new(PowMod::pow_mod(*self, exponent, *modulus))
    }
}
impl<T> PowMod for std::sync::Arc<T>
where
    T: PowMod + Copy,
{
    fn mul_mod(self, other: Self, modulus: Self) -> Self {
        std::sync::Arc::new(PowMod::mul_mod(*self, *other, *modulus))
    }
    fn pow_mod(self, exponent: u128, modulus: Self) -> Self {
        std::sync::Arc::new(PowMod::pow_mod(*self, exponent, *modulus))
    }
}
impl<T> PowMod for std::sync::Mutex<T>
where
    T: PowMod + Copy,
{
    fn mul_mod(self, other: Self, modulus: Self) -> Self {
        std::sync::Mutex::new(PowMod::mul_mod(
            *self.lock().unwrap(),
            *other.lock().unwrap(),
            *modulus.lock().unwrap(),
        ))
    }
    fn pow_mod(self, exponent: u128, modulus: Self) -> Self {
        std::sync::Mutex::new(PowMod::pow_mod(
            *self.lock().unwrap(),
            exponent,
            *modulus.lock().unwrap(),
        ))
    }
}
pub trait PowModRef {
    fn mul_mod(&self, other: Self, modulus: Self) -> Self;
    fn pow_mod(&self, exponent: u128, modulus: Self) -> Self;
}
impl<T> PowModRef for T
where
    T: PowMod + Clone,
{
    fn mul_mod(&self, other: Self, modulus: Self) -> Self {
        PowMod::mul_mod(self.clone(), other, modulus)
    }
    fn pow_mod(&self, exponent: u128, modulus: Self) -> Self {
        PowMod::pow_mod(self.clone(), exponent, modulus)
    }
}
macro_rules! miller_rabin_is_witness {
    ($base:expr,$prime:expr) => {{
        let base = $base;
        let prime = $prime;
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        miller_rabin_is_witness!(base, prime, exponent, squares)
    }};
    ($base:expr,$prime:expr,$exponent:expr,$squares:expr) => {{
        let base = $base;
        let prime = $prime;
        let exponent = $exponent;
        let squares = $squares;
        let mut result = base.pow_mod(exponent, prime);
        if result == 1 || result == prime - 1 {
            false
        } else if result == 0 {
            true
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
pub trait PrimalityTests {
    fn miller_rabin_test(self) -> bool;
    fn miller_rabin_test_iter(self, iterations: usize) -> bool;
}
impl PrimalityTests for u8 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if miller_rabin_is_witness!(2, prime) {
            return false;
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
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
        if miller_rabin_is_witness!(2, prime) {
            return false;
        }
        true
    }
}
impl PrimalityTests for i8 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if miller_rabin_is_witness!(2, prime) {
            return false;
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
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
        if miller_rabin_is_witness!(2, prime) {
            return false;
        }
        true
    }
}
impl PrimalityTests for u16 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if miller_rabin_is_witness!(7, prime, exponent, squares) {
            return false;
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
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
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if miller_rabin_is_witness!(7, prime, exponent, squares) {
            return false;
        }
        true
    }
}
impl PrimalityTests for i16 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if miller_rabin_is_witness!(7, prime, exponent, squares) {
            return false;
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
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
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if miller_rabin_is_witness!(7, prime, exponent, squares) {
            return false;
        }
        true
    }
}
impl PrimalityTests for u32 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if miller_rabin_is_witness!(7, prime, exponent, squares) {
            return false;
        }
        if miller_rabin_is_witness!(61, prime, exponent, squares) {
            return false;
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
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
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if miller_rabin_is_witness!(7, prime, exponent, squares) {
            return false;
        }
        if iterations == 2 {
            return true;
        }
        if miller_rabin_is_witness!(61, prime, exponent, squares) {
            return false;
        }
        true
    }
}
impl PrimalityTests for i32 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if miller_rabin_is_witness!(7, prime, exponent, squares) {
            return false;
        }
        if miller_rabin_is_witness!(61, prime, exponent, squares) {
            return false;
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
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
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if miller_rabin_is_witness!(7, prime, exponent, squares) {
            return false;
        }
        if iterations == 2 {
            return true;
        }
        if miller_rabin_is_witness!(61, prime, exponent, squares) {
            return false;
        }
        true
    }
}
impl PrimalityTests for u64 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as u64 {
            let prime = prime as u32;
            if miller_rabin_is_witness!(7, prime, exponent, squares) {
                return false;
            }
            if miller_rabin_is_witness!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2) {
            if miller_rabin_is_witness!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
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
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as u64 {
            let prime = prime as u32;
            if miller_rabin_is_witness!(7, prime, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_is_witness!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2).take(iterations - 1) {
            if miller_rabin_is_witness!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl PrimalityTests for i64 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as i64 {
            let prime = prime as u32;
            if miller_rabin_is_witness!(7, prime, exponent, squares) {
                return false;
            }
            if miller_rabin_is_witness!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2) {
            if miller_rabin_is_witness!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
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
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as i64 {
            let prime = prime as u32;
            if miller_rabin_is_witness!(7, prime, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_is_witness!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2).take(iterations - 1) {
            if miller_rabin_is_witness!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl PrimalityTests for u128 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as u128 {
            let prime = prime as u32;
            if miller_rabin_is_witness!(7, prime, exponent, squares) {
                return false;
            }
            if miller_rabin_is_witness!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        if prime <= u64::MAX as u128 {
            let prime = prime as u64;
            for a in (3..=43).step_by(2) {
                if miller_rabin_is_witness!(a, prime, exponent, squares) {
                    return false;
                }
            }
            return true;
        }
        for a in 3..(2 * (prime.ilog2() + 1) * (prime.ilog2() + 1)) as u128 {
            if miller_rabin_is_witness!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
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
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as u128 {
            let prime = prime as u32;
            if miller_rabin_is_witness!(7, prime, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_is_witness!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        if prime <= u64::MAX as u128 {
            let prime = prime as u64;
            for a in (3..=43).step_by(2).take(iterations - 1) {
                if miller_rabin_is_witness!(a, prime, exponent, squares) {
                    return false;
                }
            }
            return true;
        }
        for a in (3..(2 * (prime.ilog2() + 1) * (prime.ilog2() + 1)) as u128).take(iterations - 1) {
            if miller_rabin_is_witness!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl PrimalityTests for i128 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as i128 {
            let prime = prime as u32;
            if miller_rabin_is_witness!(7, prime, exponent, squares) {
                return false;
            }
            if miller_rabin_is_witness!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        if prime <= u64::MAX as i128 {
            let prime = prime as u64;
            for a in (3..=43).step_by(2) {
                if miller_rabin_is_witness!(a, prime, exponent, squares) {
                    return false;
                }
            }
            return true;
        }
        for a in 3..(2 * (prime.ilog2() + 1) * (prime.ilog2() + 1)) as i128 {
            if miller_rabin_is_witness!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
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
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as i128 {
            let prime = prime as u32;
            if miller_rabin_is_witness!(7, prime, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_is_witness!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        if prime <= u64::MAX as i128 {
            let prime = prime as u64;
            for a in (3..=43).step_by(2).take(iterations - 1) {
                if miller_rabin_is_witness!(a, prime, exponent, squares) {
                    return false;
                }
            }
            return true;
        }
        for a in (3..(2 * (prime.ilog2() + 1) * (prime.ilog2() + 1)) as i128).take(iterations - 1) {
            if miller_rabin_is_witness!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl PrimalityTests for usize {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as usize {
            let prime = prime as u32;
            if miller_rabin_is_witness!(7, prime, exponent, squares) {
                return false;
            }
            if miller_rabin_is_witness!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2) {
            if miller_rabin_is_witness!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
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
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as usize {
            let prime = prime as u32;
            if miller_rabin_is_witness!(7, prime, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_is_witness!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2).take(iterations - 1) {
            if miller_rabin_is_witness!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl PrimalityTests for isize {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as isize {
            let prime = prime as u32;
            if miller_rabin_is_witness!(7, prime, exponent, squares) {
                return false;
            }
            if miller_rabin_is_witness!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2) {
            if miller_rabin_is_witness!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
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
        let exponent = (prime >> 1) as u128;
        let squares = exponent.trailing_zeros();
        let exponent = exponent >> squares;
        if miller_rabin_is_witness!(2, prime, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as isize {
            let prime = prime as u32;
            if miller_rabin_is_witness!(7, prime, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_is_witness!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        for a in (3..=43).step_by(2).take(iterations - 1) {
            if miller_rabin_is_witness!(a, prime, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl PrimalityTests for num_bigint::BigUint {
    fn miller_rabin_test(self) -> bool {
        use num::ToPrimitive;
        use num_bigint::ToBigUint;
        let prime = self;
        if prime < 2.to_biguint().unwrap() {
            return false;
        }
        if prime.clone() % 6.to_biguint().unwrap() != 1.to_biguint().unwrap()
            && prime.clone() % 6.to_biguint().unwrap() != 5.to_biguint().unwrap()
        {
            return prime < 4.to_biguint().unwrap();
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
            let exponent = exponent.to_u128().unwrap();
            if miller_rabin_is_witness!(7, prime, exponent, squares) {
                return false;
            }
            if miller_rabin_is_witness!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        if prime <= u64::MAX.to_biguint().unwrap() {
            let prime = prime.to_u64().unwrap();
            let exponent = exponent.to_u128().unwrap();
            for a in (3..43).step_by(2) {
                if miller_rabin_is_witness!(a, prime, exponent, squares) {
                    return false;
                }
            }
            return true;
        }
        let mut a = 3.to_biguint().unwrap();
        let max_witness = (2 * (prime.bits() + 1) * (prime.bits() + 1))
            .to_biguint()
            .unwrap();
        while a <= max_witness {
            result = a.modpow(&exponent, &prime);
            if result != 1.to_biguint().unwrap()
                && result != prime.clone() - 1.to_biguint().unwrap()
            {
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
            a += 1.to_biguint().unwrap();
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        use num::ToPrimitive;
        use num_bigint::ToBigUint;
        let prime = self;
        if prime < 2.to_biguint().unwrap() {
            return false;
        }
        if prime.clone() % 6.to_biguint().unwrap() != 1.to_biguint().unwrap()
            && prime.clone() % 6.to_biguint().unwrap() != 5.to_biguint().unwrap()
        {
            return prime < 4.to_biguint().unwrap();
        }
        if iterations == 0 {
            return true;
        }
        let exponent: num::BigUint = prime.clone() >> 1;
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
        if prime < 2047.to_biguint().unwrap() || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX.to_biguint().unwrap() {
            let prime = prime.to_u32().unwrap();
            let exponent = exponent.to_u128().unwrap();
            if miller_rabin_is_witness!(7, prime, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_is_witness!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        if prime <= u64::MAX.to_biguint().unwrap() {
            let prime = prime.to_u64().unwrap();
            let exponent = exponent.to_u128().unwrap();
            for a in (3..43).step_by(2).take(iterations - 1) {
                if miller_rabin_is_witness!(a, prime, exponent, squares) {
                    return false;
                }
            }
            return true;
        }
        let mut a = 3.to_biguint().unwrap();
        let max_witness = (2 * (prime.bits() + 1) * (prime.bits() + 1))
            .to_biguint()
            .unwrap();
        let mut iterations = iterations;
        while a <= max_witness && iterations > 1 {
            result = a.modpow(&exponent, &prime);
            if result != 1.to_biguint().unwrap()
                && result != prime.clone() - 1.to_biguint().unwrap()
            {
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
            a += 1.to_biguint().unwrap();
            iterations -= 1;
        }
        true
    }
}
impl PrimalityTests for num_bigint::BigInt {
    fn miller_rabin_test(self) -> bool {
        use num::ToPrimitive;
        use num_bigint::ToBigInt;
        let prime = self;
        if prime < 2.to_bigint().unwrap() {
            return false;
        }
        if prime.clone() % 6.to_bigint().unwrap() != 1.to_bigint().unwrap()
            && prime.clone() % 6.to_bigint().unwrap() != 5.to_bigint().unwrap()
        {
            return prime < 4.to_bigint().unwrap();
        }
        let exponent: num_bigint::BigInt = prime.clone() >> 1;
        let squares = exponent.trailing_zeros().unwrap();
        let exponent = exponent >> squares;
        let mut result = 2.to_bigint().unwrap().modpow(&exponent, &prime);
        if result != 1.to_bigint().unwrap() && result != prime.clone() - 1.to_bigint().unwrap() {
            let mut is_witness = true;
            for _ in 0..squares {
                result = result.clone() * result.clone() % prime.clone();
                if result < 2.to_bigint().unwrap() {
                    return false;
                }
                if result == prime.clone() - 1.to_bigint().unwrap() {
                    is_witness = false;
                    break;
                }
            }
            if is_witness {
                return false;
            }
        }
        if prime < 2047.to_bigint().unwrap() {
            return true;
        }
        if prime <= u32::MAX.to_bigint().unwrap() {
            let prime = prime.to_u32().unwrap();
            let exponent = exponent.to_u128().unwrap();
            if miller_rabin_is_witness!(7, prime, exponent, squares) {
                return false;
            }
            if miller_rabin_is_witness!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        if prime <= u64::MAX.to_bigint().unwrap() {
            let prime = prime.to_u64().unwrap();
            let exponent = exponent.to_u128().unwrap();
            for a in (3..43).step_by(2) {
                if miller_rabin_is_witness!(a, prime, exponent, squares) {
                    return false;
                }
            }
            return true;
        }
        let mut a = 3.to_bigint().unwrap();
        let max_witness = (2 * (prime.bits() + 1) * (prime.bits() + 1))
            .to_bigint()
            .unwrap();
        while a <= max_witness {
            result = a.modpow(&exponent, &prime);
            if result != 1.to_bigint().unwrap() && result != prime.clone() - 1.to_bigint().unwrap()
            {
                let mut is_witness = true;
                for _ in 0..squares {
                    result = result.clone() * result.clone() % prime.clone();
                    if result < 2.to_bigint().unwrap() {
                        return false;
                    }
                    if result == prime.clone() - 1.to_bigint().unwrap() {
                        is_witness = false;
                        break;
                    }
                }
                if is_witness {
                    return false;
                }
            }
            a += 1.to_bigint().unwrap();
        }
        true
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        use num::ToPrimitive;
        use num_bigint::ToBigInt;
        let prime = self;
        if prime < 2.to_bigint().unwrap() {
            return false;
        }
        if prime.clone() % 6.to_bigint().unwrap() != 1.to_bigint().unwrap()
            && prime.clone() % 6.to_bigint().unwrap() != 5.to_bigint().unwrap()
        {
            return prime < 4.to_bigint().unwrap();
        }
        if iterations == 0 {
            return true;
        }
        let exponent: num_bigint::BigInt = prime.clone() >> 1;
        let squares = exponent.trailing_zeros().unwrap();
        let exponent = exponent >> squares;
        let mut result = 2.to_bigint().unwrap().modpow(&exponent, &prime);
        if result != 1.to_bigint().unwrap() && result != prime.clone() - 1.to_bigint().unwrap() {
            let mut is_witness = true;
            for _ in 0..squares {
                result = result.clone() * result.clone() % prime.clone();
                if result < 2.to_bigint().unwrap() {
                    return false;
                }
                if result == prime.clone() - 1.to_bigint().unwrap() {
                    is_witness = false;
                    break;
                }
            }
            if is_witness {
                return false;
            }
        }
        if prime < 2047.to_bigint().unwrap() || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX.to_bigint().unwrap() {
            let prime = prime.to_u32().unwrap();
            let exponent = exponent.to_u128().unwrap();
            if miller_rabin_is_witness!(7, prime, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_is_witness!(61, prime, exponent, squares) {
                return false;
            }
            return true;
        }
        if prime <= u64::MAX.to_bigint().unwrap() {
            let prime = prime.to_u64().unwrap();
            let exponent = exponent.to_u128().unwrap();
            for a in (3..43).step_by(2).take(iterations - 1) {
                if miller_rabin_is_witness!(a, prime, exponent, squares) {
                    return false;
                }
            }
            return true;
        }
        let mut a = 3.to_bigint().unwrap();
        let max_witness = (2 * (prime.bits() + 1) * (prime.bits() + 1))
            .to_bigint()
            .unwrap();
        let mut iterations = iterations;
        while a <= max_witness && iterations > 1 {
            result = a.modpow(&exponent, &prime);
            if result != 1.to_bigint().unwrap() && result != prime.clone() - 1.to_bigint().unwrap()
            {
                let mut is_witness = true;
                for _ in 0..squares {
                    result = result.clone() * result.clone() % prime.clone();
                    if result < 2.to_bigint().unwrap() {
                        return false;
                    }
                    if result == prime.clone() - 1.to_bigint().unwrap() {
                        is_witness = false;
                        break;
                    }
                }
                if is_witness {
                    return false;
                }
            }
            a += 1.to_bigint().unwrap();
            iterations -= 1;
        }
        true
    }
}
impl<T> PrimalityTests for &T
where
    T: PrimalityTests + Clone,
{
    fn miller_rabin_test(self) -> bool {
        PrimalityTests::miller_rabin_test(self.clone())
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        PrimalityTests::miller_rabin_test_iter(self.clone(), iterations)
    }
}
impl<T> PrimalityTests for std::rc::Rc<T>
where
    T: PrimalityTests + Clone,
{
    fn miller_rabin_test(self) -> bool {
        PrimalityTests::miller_rabin_test(self.as_ref())
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        PrimalityTests::miller_rabin_test_iter(self.as_ref(), iterations)
    }
}
impl<T> PrimalityTests for std::sync::Arc<T>
where
    T: PrimalityTests + Clone,
{
    fn miller_rabin_test(self) -> bool {
        PrimalityTests::miller_rabin_test(self.as_ref())
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        PrimalityTests::miller_rabin_test_iter(self.as_ref(), iterations)
    }
}
impl<T> PrimalityTests for std::sync::Mutex<T>
where
    T: PrimalityTests + Clone,
{
    fn miller_rabin_test(self) -> bool {
        PrimalityTests::miller_rabin_test(self.lock().unwrap().clone())
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        PrimalityTests::miller_rabin_test_iter(self.lock().unwrap().clone(), iterations)
    }
}
#[cfg(test)]
mod test {
    use crate::math::primality::*;
    macro_rules! pow_mod {
        ($type:ty) => {
            let result = PowMod::mul_mod(4, 6, <$type>::MAX);
            assert_eq!(
                result,
                24,
                "Expected <{0} as PowMod>::mul_mod(4,6,{0}::MAX) to be equal to 24. Got {result}",
                stringify!($type)
            );
            let result = PowModRef::mul_mod(&4, 6, <$type>::MAX);
            assert_eq!(
                result,
                24,
                "Expected <{0} as PowModRef>::mul_mod(4,6,{0}::MAX) to be equal to 24. Got {result}",
                stringify!($type)
            );
            let result = PowMod::mul_mod(<$type>::MAX-1,<$type>::MAX-1,<$type>::MAX);
            assert_eq!(result,1,"Expected <{0} as PowMod>::mul_mod({0}::MAX-1,{0}::MAX-1,{0}::MAX) to be equal to 1. Got {result}",stringify!($type));
            let result=PowMod::pow_mod(2 as $type,8,9);
            assert_eq!(result,4,"Expected <{0} as PowMod>::pow_mod(2,8,9) to be equal to 4. Got {result}",stringify!($type));
            let result=PowMod::pow_mod(2 as $type,26,27);
            assert_eq!(result,13,"Expected <{0} as PowMod>::pow_mod(2,26,27) to be equal to 13. Got {result}",stringify!($type));
        };
    }
    fn pow_mod() {
        pow_mod!(u8);
        pow_mod!(i8);
        pow_mod!(u16);
        pow_mod!(i16);
        pow_mod!(u32);
        pow_mod!(i32);
        pow_mod!(u64);
        pow_mod!(i64);
        pow_mod!(u128);
        pow_mod!(i128);
        pow_mod!(usize);
        pow_mod!(isize);
    }
    macro_rules! primality_tests {
        ($type:ty) => {
            let result=(0..127).filter(|x|<&$type as PrimalityTests>::miller_rabin_test(x)).collect::<Vec<_>>();
            assert_eq!(result,[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113], "Expected <{} as PrimalityTests>::miller_rabin_test> to filter correctly.",stringify!($type))
        };
        ($type:ty,$($number:expr,$iterations:expr,$is_prime:expr),+) => {
            primality_tests!($type);
            $(assert!(<$type as PrimalityTests>::miller_rabin_test_iter($number,$iterations)==$is_prime));+
        };
    }
    fn primality_tests() {
        use num_bigint::{ToBigInt, ToBigUint};
        primality_tests!(u8);
        primality_tests!(i8);
        primality_tests!(u16, 2047, 0, true, 2047, 1, true, 2047, 2, false);
        primality_tests!(i16, 2047, 0, true, 2047, 1, true, 2047, 2, false);
        primality_tests!(u32, 2047, 0, true, 2047, 1, true, 2047, 2, false);
        primality_tests!(i32, 2047, 0, true, 2047, 1, true, 2047, 2, false);
        primality_tests!(u64, 2047, 0, true, 2047, 1, true, 2047, 2, false);
        primality_tests!(i64, 2047, 0, true, 2047, 1, true, 2047, 2, false);
        primality_tests!(u128, 2047, 0, true, 2047, 1, true, 2047, 2, false);
        primality_tests!(i128, 2047, 0, true, 2047, 1, true, 2047, 2, false);
        primality_tests!(usize, 2047, 0, true, 2047, 1, true, 2047, 2, false);
        primality_tests!(isize, 2047, 0, true, 2047, 1, true, 2047, 2, false);
        let result = (0..0x100)
            .filter(|x| x.to_biguint().unwrap().miller_rabin_test())
            .collect::<Vec<_>>();
        assert_eq!(
            result,
            [
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251
            ],
            "Expected <BigUint as PrimailityTests> to filter correctly"
        );
        assert!(2047.to_biguint().unwrap().miller_rabin_test_iter(0));
        assert!(2047.to_biguint().unwrap().miller_rabin_test_iter(1));
        assert!(!2047.to_biguint().unwrap().miller_rabin_test_iter(2));
        let result = (0..0x100)
            .filter(|x| x.to_bigint().unwrap().miller_rabin_test())
            .collect::<Vec<_>>();
        assert_eq!(
            result,
            [
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251
            ],
            "Expected <BigInt as PrimailityTests> to filter correctly"
        );
        assert!(2047.to_bigint().unwrap().miller_rabin_test_iter(0));
        assert!(2047.to_bigint().unwrap().miller_rabin_test_iter(1));
        assert!(!2047.to_bigint().unwrap().miller_rabin_test_iter(2));
    }
    #[test]
    fn primality() {
        pow_mod();
        primality_tests();
    }
}
