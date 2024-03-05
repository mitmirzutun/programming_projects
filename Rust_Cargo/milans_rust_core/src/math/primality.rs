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
        let exponent = prime >> 1;
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
        let exponent = prime >> 1;
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
macro_rules! count {
    ($type:ty,$pos:expr) => {{
        let pos = $pos;
        let diff = <$type>::MAX - pos;
        if diff > 30 {
            match pos % 30 {
                1 | 23 => 6,
                2 | 24 => 5,
                3 | 7 | 13 | 19 | 25 => 4,
                4 | 8 | 14 | 20 | 26 => 3,
                5 | 9 | 11 | 15 | 17 | 21 | 27 | 29 => 2,
                _ => 1,
            }
        } else if diff > 6 {
            match pos % 6 {
                1 => 4,
                2 => 3,
                3 | 5 => 2,
                _ => 1,
            }
        } else {
            if pos % 2 == 0 {
                1
            } else {
                2
            }
        }
    }};
}
macro_rules! prime_generator {
    ($type:ty) => {
        prime_generator_from!($type);
        prime_generator_trait!($type);
        prime_generator_iter!($type);
    };
}
macro_rules! prime_generator_from {
    ($type:ty) => {
        impl From<($type, $type, Option<usize>)> for PrimeGenerator<$type> {
            fn from((pos, stop, iterations): ($type, $type, Option<usize>)) -> Self {
                Self {
                    pos,
                    stop,
                    iterations,
                }
            }
        }
    };
}
macro_rules! prime_generator_trait {
    ($type:ty) => {
        impl PrimeGeneratorTrait<$type> for PrimeGenerator<$type> {
            fn set_iterations(&mut self, iterations: Option<usize>) -> &mut Self {
                self.iterations = iterations;
                self
            }
            fn get_iterations(&self) -> Option<usize> {
                self.iterations
            }
            fn get_pos(&self) -> $type {
                self.pos
            }
            fn get_stop(&self) -> $type {
                self.stop
            }
            fn next_n_numbers(&mut self, number: u64) -> Option<Vec<$type>> {
                if self.pos >= self.stop {
                    return None;
                }
                let pos = <$type>::max(0, self.pos);
                let diff = u64::min(number, (<$type>::MAX - pos) as u64);
                let stop = <$type>::min(self.stop, pos + diff as $type);
                self.pos = stop;
                Some(PrimeGenerator::from((pos, stop, self.iterations)).collect())
            }
            fn next_n_primes(&mut self, number: usize) -> Option<Vec<$type>> {
                let mut result = vec![];
                if self.pos <= 2 && self.stop > 2 && number > result.len() {
                    result.push(2);
                    self.pos = 3
                }
                if self.pos <= 3 && self.stop > 2 && number > result.len() {
                    result.push(3);
                    self.pos = 5;
                }
                if self.pos <= 5 && number > result.len() {
                    result.push(5);
                    self.pos = 7;
                }
                while result.len() < number {
                    if self.pos >= self.stop {
                        if result.len() == 0 {
                            return None;
                        }
                        return Some(result);
                    }
                    if self.iterations.is_some()
                        && self.pos.miller_rabin_test_iter(self.iterations.unwrap())
                        || self.iterations.is_none() && self.pos.miller_rabin_test()
                    {
                        result.push(self.pos)
                    }
                    self.pos += count!($type, self.pos)
                }
                Some(result)
            }
        }
    };
}
macro_rules! prime_generator_iter {
    ($type:ty) => {
        impl Iterator for PrimeGenerator<$type> {
            type Item = $type;
            fn next(&mut self) -> Option<$type> {
                use $crate::math::primality::PrimalityTests;
                if self.pos <= 2 && self.stop > 2 {
                    self.pos = 3;
                    return Some(2);
                }
                if self.pos <= 3 && self.stop > 3 {
                    self.pos = 5;
                    return Some(3);
                }
                if self.pos <= 5 && self.stop > 5 {
                    self.pos = 7;
                    return Some(5);
                }
                if let Some(iterations) = self.iterations {
                    loop {
                        if self.pos >= self.stop {
                            return None;
                        }
                        if self.pos.miller_rabin_test_iter(iterations) {
                            let result = self.pos;
                            self.pos += count!($type, self.pos);
                            return Some(result);
                        }
                        self.pos += count!($type, self.pos);
                    }
                } else {
                    loop {
                        if self.pos >= self.stop {
                            return None;
                        }
                        if self.pos.miller_rabin_test() {
                            let result = self.pos;
                            self.pos += count!($type, self.pos);
                            return Some(result);
                        }
                        self.pos += count!($type, self.pos);
                    }
                }
            }
        }
    };
}
macro_rules! async_prime_generator {
    ($type:ty) => {
        prime_generator!($type);
        async_prime_generator_from!($type);
        async_prime_generator_trait!($type);
        async_prime_generator_iter!($type);
    };
}
macro_rules! async_prime_generator_from {
    ($type:ty) => {
        impl From<($type, $type, Option<usize>)> for AsyncPrimeGenerator<$type> {
            fn from((pos, stop, iterations): ($type, $type, Option<usize>)) -> Self {
                Self {
                    pos: std::sync::Arc::new(std::sync::Mutex::new(pos)),
                    stop: std::sync::Arc::new(stop),
                    iterations,
                }
            }
        }
    };
}
macro_rules! async_prime_generator_trait {
    ($type:ty) => {
        impl PrimeGeneratorTrait<$type> for AsyncPrimeGenerator<$type> {
            fn set_iterations(&mut self, iterations: Option<usize>) -> &mut Self {
                self.iterations = iterations;
                self
            }
            fn get_iterations(&self) -> Option<usize> {
                self.iterations
            }
            fn get_pos(&self) -> $type {
                *self.pos.lock().unwrap()
            }
            fn get_stop(&self) -> $type {
                *self.stop
            }
            fn next_n_numbers(&mut self, number: u64) -> Option<Vec<$type>> {
                let mut self_pos = self.pos.lock().unwrap();
                if *self_pos >= *self.stop {
                    return None;
                }
                let pos = <$type>::max(0, *self_pos);
                let diff = u64::min(number, (<$type>::MAX - pos) as u64);
                let stop = <$type>::min(*self.stop, pos + diff as $type);
                *self_pos = stop;
                Some(PrimeGenerator::from((pos, stop, self.iterations)).collect())
            }
            fn next_n_primes(&mut self, number: usize) -> Option<Vec<$type>> {
                let mut result = vec![];
                let mut pos = self.pos.lock().unwrap();
                if *pos <= 2 && *self.stop > 2 && number > result.len() {
                    result.push(2);
                    *pos = 3
                }
                if *pos <= 3 && *self.stop > 2 && number > result.len() {
                    result.push(3);
                    *pos = 5;
                }
                if *pos <= 5 && number > result.len() {
                    result.push(5);
                    *pos = 7;
                }
                while result.len() < number {
                    if *pos >= *self.stop {
                        if result.len() == 0 {
                            return None;
                        }
                        return Some(result);
                    }
                    if self.iterations.is_some()
                        && pos.miller_rabin_test_iter(self.iterations.unwrap())
                        || self.iterations.is_none() && pos.miller_rabin_test()
                    {
                        result.push(*pos)
                    }
                    *pos += count!($type, *pos)
                }
                Some(result)
            }
        }
        impl AsyncPrimeGeneratorTrait<$type> for AsyncPrimeGenerator<$type> {
            fn count(&mut self, num_threads: Option<std::num::NonZeroU8>) -> $type {
                todo!()
            }
            fn collect(&mut self, num_threads: Option<std::num::NonZeroU8>) -> Vec<$type> {
                todo!()
            }
        }
    };
}
macro_rules! async_prime_generator_iter {
    ($type:ty) => {
        impl Iterator for AsyncPrimeGenerator<$type> {
            type Item = $type;
            fn next(&mut self) -> Option<$type> {
                let mut pos = self.pos.lock().unwrap();
                use $crate::math::primality::PrimalityTests;
                if *pos <= 2 && *self.stop > 2 {
                    *pos = 3;
                    return Some(2);
                }
                if *pos <= 3 && *self.stop > 3 {
                    *pos = 5;
                    return Some(3);
                }
                if *pos <= 5 && *self.stop > 5 {
                    *pos = 7;
                    return Some(5);
                }
                if let Some(iterations) = self.iterations {
                    loop {
                        if *pos >= *self.stop {
                            return None;
                        }
                        if pos.miller_rabin_test_iter(iterations) {
                            let result = *pos;
                            *pos += count!($type, *pos);
                            return Some(result);
                        }
                        *pos += count!($type, *pos);
                    }
                } else {
                    loop {
                        if *pos >= *self.stop {
                            return None;
                        }
                        if pos.miller_rabin_test() {
                            let result = *pos;
                            *pos += count!($type, *pos);
                            return Some(result);
                        }
                        *pos += count!($type, *pos);
                    }
                }
            }
        }
    };
}
pub trait PrimeGeneratorTrait<T>
where
    T: PrimalityTests,
{
    fn set_iterations(&mut self, iterations: Option<usize>) -> &mut Self;
    fn get_iterations(&self) -> Option<usize>;
    fn get_stop(&self) -> T;
    fn get_pos(&self) -> T;
    fn next_n_primes(&mut self, number: usize) -> Option<Vec<T>>;
    fn next_n_numbers(&mut self, number: u64) -> Option<Vec<T>>;
}
pub struct PrimeGenerator<T>
where
    T: PrimalityTests,
    PrimeGenerator<T>: Iterator<Item = T> + PrimeGeneratorTrait<T>,
{
    pos: T,
    stop: T,
    iterations: Option<usize>,
}
impl<T> From<T> for PrimeGenerator<T>
where
    T: PrimalityTests + From<u8>,
    PrimeGenerator<T>: Iterator<Item = T> + PrimeGeneratorTrait<T> + From<(T, T, Option<usize>)>,
{
    fn from(value: T) -> Self {
        Self::from((0.into(), value, None))
    }
}
impl<T> From<(T, T)> for PrimeGenerator<T>
where
    T: PrimalityTests,
    PrimeGenerator<T>: Iterator<Item = T> + PrimeGeneratorTrait<T> + From<(T, T, Option<usize>)>,
{
    fn from((start, stop): (T, T)) -> Self {
        Self::from((start, stop, None))
    }
}
impl<T> From<(T, Option<usize>)> for PrimeGenerator<T>
where
    T: PrimalityTests + From<u8>,
    PrimeGenerator<T>: Iterator<Item = T> + PrimeGeneratorTrait<T> + From<(T, T, Option<usize>)>,
{
    fn from((stop, iterations): (T, Option<usize>)) -> Self {
        Self::from((0.into(), stop, iterations))
    }
}
impl<T> From<T> for AsyncPrimeGenerator<T>
where
    T: PrimalityTests + From<u8>,
    AsyncPrimeGenerator<T>: Iterator<Item = T>
        + PrimeGeneratorTrait<T>
        + From<(T, T, Option<usize>)>
        + AsyncPrimeGeneratorTrait<T>,
{
    fn from(value: T) -> Self {
        Self::from((0.into(), value, None))
    }
}
impl<T> From<(T, T)> for AsyncPrimeGenerator<T>
where
    T: PrimalityTests,
    AsyncPrimeGenerator<T>: Iterator<Item = T>
        + PrimeGeneratorTrait<T>
        + From<(T, T, Option<usize>)>
        + AsyncPrimeGeneratorTrait<T>,
{
    fn from((start, stop): (T, T)) -> Self {
        Self::from((start, stop, None))
    }
}
impl<T> From<(T, Option<usize>)> for AsyncPrimeGenerator<T>
where
    T: PrimalityTests + From<u8>,
    AsyncPrimeGenerator<T>: Iterator<Item = T>
        + PrimeGeneratorTrait<T>
        + From<(T, T, Option<usize>)>
        + AsyncPrimeGeneratorTrait<T>,
{
    fn from((stop, iterations): (T, Option<usize>)) -> Self {
        Self::from((0.into(), stop, iterations))
    }
}
pub trait AsyncPrimeGeneratorTrait<T>
where
    T: PrimalityTests,
{
    fn count(&mut self, num_threads: Option<std::num::NonZeroU8>) -> T;
    fn collect(&mut self, num_threads: Option<std::num::NonZeroU8>) -> Vec<T>;
}
pub struct AsyncPrimeGenerator<T>
where
    T: PrimalityTests,
    AsyncPrimeGenerator<T>:
        Iterator<Item = T> + PrimeGeneratorTrait<T> + AsyncPrimeGeneratorTrait<T>,
{
    pos: std::sync::Arc<std::sync::Mutex<T>>,
    stop: std::sync::Arc<T>,
    iterations: Option<usize>,
}
async_prime_generator!(u8);
async_prime_generator!(i8);
async_prime_generator!(u16);
async_prime_generator!(i16);
async_prime_generator!(u32);
async_prime_generator!(i32);
async_prime_generator!(u64);
async_prime_generator!(i64);
async_prime_generator!(u128);
async_prime_generator!(i128);
async_prime_generator!(usize);
async_prime_generator!(isize);
prime_generator_from!(num::BigUint);
prime_generator_from!(num::BigInt);
impl PrimeGeneratorTrait<num_bigint::BigUint> for PrimeGenerator<num_bigint::BigUint> {
    fn set_iterations(&mut self, iterations: Option<usize>) -> &mut Self {
        self.iterations = iterations;
        self
    }
    fn get_iterations(&self) -> Option<usize> {
        self.iterations
    }
    fn get_pos(&self) -> num_bigint::BigUint {
        self.pos.clone()
    }
    fn get_stop(&self) -> num_bigint::BigUint {
        self.stop.clone()
    }
    fn next_n_numbers(&mut self, number: u64) -> Option<Vec<num_bigint::BigUint>> {
        todo!()
    }
    fn next_n_primes(&mut self, number: usize) -> Option<Vec<num_bigint::BigUint>> {
        todo!()
    }
}
impl PrimeGeneratorTrait<num_bigint::BigInt> for PrimeGenerator<num_bigint::BigInt> {
    fn set_iterations(&mut self, iterations: Option<usize>) -> &mut Self {
        self.iterations = iterations;
        self
    }
    fn get_iterations(&self) -> Option<usize> {
        self.iterations
    }
    fn get_pos(&self) -> num_bigint::BigInt {
        self.pos.clone()
    }
    fn get_stop(&self) -> num_bigint::BigInt {
        self.stop.clone()
    }
    fn next_n_numbers(&mut self, number: u64) -> Option<Vec<num_bigint::BigInt>> {
        todo!()
    }
    fn next_n_primes(&mut self, number: usize) -> Option<Vec<num_bigint::BigInt>> {
        todo!()
    }
}
impl Iterator for PrimeGenerator<num_bigint::BigUint> {
    type Item = num_bigint::BigUint;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
impl Iterator for PrimeGenerator<num_bigint::BigInt> {
    type Item = num_bigint::BigInt;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
async_prime_generator_from!(num_bigint::BigUint);
async_prime_generator_from!(num_bigint::BigInt);
impl PrimeGeneratorTrait<num_bigint::BigUint> for AsyncPrimeGenerator<num_bigint::BigUint> {
    fn set_iterations(&mut self, iterations: Option<usize>) -> &mut Self {
        self.iterations = iterations;
        self
    }
    fn get_iterations(&self) -> Option<usize> {
        self.iterations
    }
    fn get_pos(&self) -> num_bigint::BigUint {
        self.pos.lock().unwrap().clone()
    }
    fn get_stop(&self) -> num_bigint::BigUint {
        self.stop.as_ref().clone()
    }
    fn next_n_numbers(&mut self, number: u64) -> Option<Vec<num_bigint::BigUint>> {
        todo!()
    }
    fn next_n_primes(&mut self, number: usize) -> Option<Vec<num_bigint::BigUint>> {
        todo!()
    }
}
impl PrimeGeneratorTrait<num_bigint::BigInt> for AsyncPrimeGenerator<num_bigint::BigInt> {
    fn set_iterations(&mut self, iterations: Option<usize>) -> &mut Self {
        self.iterations = iterations;
        self
    }
    fn get_iterations(&self) -> Option<usize> {
        self.iterations
    }
    fn get_pos(&self) -> num_bigint::BigInt {
        self.pos.lock().unwrap().clone()
    }
    fn get_stop(&self) -> num_bigint::BigInt {
        self.stop.as_ref().clone()
    }
    fn next_n_numbers(&mut self, number: u64) -> Option<Vec<num_bigint::BigInt>> {
        todo!()
    }
    fn next_n_primes(&mut self, number: usize) -> Option<Vec<num_bigint::BigInt>> {
        todo!()
    }
}
impl AsyncPrimeGeneratorTrait<num_bigint::BigUint> for AsyncPrimeGenerator<num_bigint::BigUint> {
    fn count(&mut self, num_threads: Option<std::num::NonZeroU8>) -> num_bigint::BigUint {
        todo!()
    }

    fn collect(&mut self, num_threads: Option<std::num::NonZeroU8>) -> Vec<num_bigint::BigUint> {
        todo!()
    }
}
impl AsyncPrimeGeneratorTrait<num_bigint::BigInt> for AsyncPrimeGenerator<num_bigint::BigInt> {
    fn count(&mut self, num_threads: Option<std::num::NonZeroU8>) -> num_bigint::BigInt {
        todo!()
    }

    fn collect(&mut self, num_threads: Option<std::num::NonZeroU8>) -> Vec<num_bigint::BigInt> {
        todo!()
    }
}
impl Iterator for AsyncPrimeGenerator<num_bigint::BigUint> {
    type Item = num_bigint::BigUint;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
impl Iterator for AsyncPrimeGenerator<num_bigint::BigInt> {
    type Item = num_bigint::BigInt;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
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
    macro_rules! prime_generator {
        ($type:ty) => {
            let result = PrimeGenerator::<$type>::from((0, 127)).collect::<Vec<_>>();
            assert_eq!(
                result,
                [
                    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
                    79, 83, 89, 97, 101, 103, 107, 109, 113
                ]
            );
            let result =
                PrimeGenerator::<$type>::from((<$type>::MIN, <$type>::MAX)).next_n_numbers(100);
            assert_eq!(
                result,
                Some(vec![
                    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
                    79, 83, 89, 97
                ]),
                "PrimeGenerator::<{}>::next_n_numbers(100) didnt generate a correct sequence",
                stringify!($type)
            );
            let result =
                PrimeGenerator::<$type>::from((<$type>::MIN, <$type>::MAX)).next_n_primes(25);
            assert_eq!(
                result,
                Some(vec![
                    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
                    79, 83, 89, 97
                ]),
                "PrimeGenerator::<{}>::next_n_primes(100) didnt generate a correct sequence",
                stringify!($type)
            );
            let result = AsyncPrimeGenerator::<$type>::from((0, 127)).collect::<Vec<_>>();
            assert_eq!(
                result,
                [
                    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
                    79, 83, 89, 97, 101, 103, 107, 109, 113
                ]
            );
            let result = AsyncPrimeGenerator::<$type>::from((<$type>::MIN, <$type>::MAX))
                .next_n_numbers(100);
            assert_eq!(
                result,
                Some(vec![
                    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
                    79, 83, 89, 97
                ]),
                "AsyncPrimeGenerator::<{}>::next_n_numbers(100) didnt generate a correct sequence",
                stringify!($type)
            );
            let result =
                AsyncPrimeGenerator::<$type>::from((<$type>::MIN, <$type>::MAX)).next_n_primes(25);
            assert_eq!(
                result,
                Some(vec![
                    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
                    79, 83, 89, 97
                ]),
                "AsyncPrimeGenerator::<{}>::next_n_primes(100) didnt generate a correct sequence",
                stringify!($type)
            );
        };
    }
    fn prime_generator() {
        prime_generator!(u8);
        prime_generator!(i8);
        prime_generator!(u16);
        prime_generator!(i16);
        prime_generator!(u32);
        prime_generator!(i32);
        prime_generator!(u64);
        prime_generator!(i64);
        prime_generator!(u128);
        prime_generator!(i128);
        prime_generator!(usize);
        prime_generator!(isize);
    }
    #[test]
    fn primality() {
        pow_mod();
        primality_tests();
        prime_generator();
    }
}
