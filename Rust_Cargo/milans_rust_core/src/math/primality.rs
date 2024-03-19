macro_rules! pow_mod {
    ($type1:ty,$type2:ty) => {
        impl PowMod for $type1 {
            fn mul_mod(self, other: $type1, modulus: $type1) -> $type1 {
                $crate::trace!(
                    "mul_mod({self}_{0}, {other}_{0}, {modulus}_{0})={1}",
                    stringify!($type1),
                    ((self as $type2 * other as $type2) % modulus as $type2) as $type1
                )
                .unwrap();
                ((self as $type2 * other as $type2) % modulus as $type2) as $type1
            }
            fn pow_mod(self, exponent: u128, modulus: $type1) -> $type1 {
                $crate::trace!(
                    "pow_mod({self}_{0}, {other}_{0}, {modulus}_{0})",
                    stringify!($type1)
                )
                .unwrap();
                let mut result = 1;
                let mut base = self;
                for bit in 0..128 {
                    if exponent & (1 << bit) > 0 {
                        result = result.mul_mod(base, modulus)
                    }
                    base = base.mul_mod(base, modulus);
                }
                $crate::trace!(
                    "pow_mod({self}_{0}, {other}_{0}, {modulus}_{0})={1}",
                    stringify!($type1),
                    result
                )
                .unwrap();
                result
            }
        }
    };
}
pub trait PowMod {
    fn mul_mod(self, other: Self, modulus: Self) -> Self;
    fn pow_mod(self, exponent: u128, modulus: Self) -> Self;
}
impl<T> PowMod for std::rc::Rc<T>
where
    T: PowMod + Copy,
{
    fn mul_mod(self, other: Self, modulus: Self) -> Self {
        std::rc::Rc::new(T::mul_mod(*self, *other, *modulus))
    }
    fn pow_mod(self, exponent: u128, modulus: Self) -> Self {
        std::rc::Rc::new(T::pow_mod(*self, exponent, *modulus))
    }
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
    fn mul_mod(self, other: u128, modulus: u128) -> u128 {
        use num::ToPrimitive;
        use num_bigint::ToBigUint;
        let result = (self.to_biguint().unwrap() * other.to_biguint().unwrap()
            % modulus.to_biguint().unwrap())
        .to_u128()
        .unwrap();
        crate::trace!(
            "mul_mod({self}_{0}, {other}_{0}, {modulus}_{0})={1}",
            stringify!(u128)
            result
        )
        .unwrap();
        result
    }
    fn pow_mod(self, exponent: u128, modulus: u128) -> u128 {
        crate::trace!(
            "pow_mod({self}_{0}, {other}_{0}, {modulus}_{0})",
            stringify!(u128)
        )
        .unwrap();
        let mut result = 1;
        let mut base = self;
        for bit in 0..128 {
            if exponent & (1 << bit) > 0 {
                result = result.mul_mod(base, modulus)
            }
            base = base.mul_mod(base, modulus);
        }
        crate::trace!(
            "pow_mod({self}_{0}, {other}_{0}, {modulus}_{0})={1}",
            stringify!(u128)
            result
        )
        .unwrap();
        result
    }
}
impl PowMod for i128 {
    fn mul_mod(self, other: i128, modulus: i128) -> i128 {
        use num::ToPrimitive;
        use num_bigint::ToBigInt;
        let result = (self.to_bigint().unwrap() * other.to_bigint().unwrap()
            % modulus.to_bigint().unwrap())
        .to_i128()
        .unwrap();
        crate::trace!(
            "mul_mod({self}_{0}, {other}_{0}, {modulus}_{0})={1}",
            stringify!(i128)
            result
        )
        .unwrap();
        result
    }
    fn pow_mod(self, exponent: u128, modulus: i128) -> i128 {
        crate::trace!(
            "mul_mod({self}_{0}, {other}_{0}, {modulus}_{0})",
            stringify!(i128)
        )
        .unwrap();
        let mut result = 1;
        let mut base = self;
        for bit in 0..128 {
            if exponent & (1 << bit) > 0 {
                result = result.mul_mod(base, modulus)
            }
            base = base.mul_mod(base, modulus);
        }
        crate::trace!(
            "pow_mod({self}_{0}, {other}_{0}, {modulus}_{0})={1}",
            stringify!(i128)
            result
        )
        .unwrap();
        result
    }
}
pow_mod!(usize, u128);
pow_mod!(isize, i128);
macro_rules! miller_rabin_test {
    ($witness:expr,$prime:expr) => {{
        let witness = $witness;
        let prime = $prime;
        let exponent = prime >> 1;
        let squares = prime.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        miller_rabin_test!(witness, prime, exponent, squares)
    }};
    ($witness:expr,$prime:expr,$exponent:expr,$squares:expr) => {{
        let witness = $witness;
        let prime = $prime;
        let exponent = $exponent;
        let squares = $squares;
        let mut result = witness.pow_mod(exponent, prime);
        $crate::debug!("{prime}=2**{squares}*{exponent} {result}={witness}**{exponent}").unwrap();
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
pub trait PrimalityTests {
    fn miller_rabin_test(self) -> bool;
    fn miller_rabin_test_iter(self, iterations: usize) -> bool;
    fn divisibility_test(self) -> bool;
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
        if miller_rabin_test!(2, prime) {
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
        if miller_rabin_test!(2, prime) {
            return false;
        }
        true
    }
    fn divisibility_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 2 == 0 || prime % 3 == 0 {
            return prime < 4;
        }
        for p in (5..=super::sqrt(prime).unwrap()).step_by(6) {
            crate::trace!("{self} {} {}", p, p + 2).unwrap();
            if prime % p == 0 || prime % (p + 2) == 0 {
                return false;
            }
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
        if miller_rabin_test!(2, prime) {
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
        if miller_rabin_test!(2, prime) {
            return false;
        }
        true
    }
    fn divisibility_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 2 == 0 || prime % 3 == 0 {
            return prime < 4;
        }
        for p in (5..=super::sqrt(prime as u8).unwrap() as i8).step_by(6) {
            crate::trace!("{self} {} {}", p, p + 2).unwrap();
            if prime % p == 0 || prime % (p + 2) == 0 {
                return false;
            }
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
    fn divisibility_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 2 == 0 || prime % 3 == 0 {
            return prime < 4;
        }
        for p in (5..=super::sqrt(prime).unwrap()).step_by(6) {
            crate::trace!("{self} {} {}", p, p + 2).unwrap();
            if prime % p == 0 || prime % (p + 2) == 0 {
                return false;
            }
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
    fn divisibility_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 2 == 0 || prime % 3 == 0 {
            return prime < 4;
        }
        for p in (5..=super::sqrt(prime).unwrap()).step_by(6) {
            crate::trace!("{self} {} {}", p, p + 2).unwrap();
            if prime % p == 0 || prime % (p + 2) == 0 {
                return false;
            }
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
    fn divisibility_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 2 == 0 || prime % 3 == 0 {
            return prime < 4;
        }
        for p in (5..=super::sqrt(prime).unwrap()).step_by(6) {
            crate::trace!("{self} {} {}", p, p + 2).unwrap();
            if prime % p == 0 || prime % (p + 2) == 0 {
                return false;
            }
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
    fn divisibility_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 2 == 0 || prime % 3 == 0 {
            return prime < 4;
        }
        for p in (5..=super::sqrt(prime).unwrap()).step_by(6) {
            crate::trace!("{self} {} {}", p, p + 2).unwrap();
            if prime % p == 0 || prime % (p + 2) == 0 {
                return false;
            }
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
                return true;
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
    fn divisibility_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 2 == 0 || prime % 3 == 0 {
            return prime < 4;
        }
        for p in (5..=super::sqrt(prime).unwrap()).step_by(6) {
            crate::trace!("{self} {} {}", p, p + 2).unwrap();
            if prime % p == 0 || prime % (p + 2) == 0 {
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
            if iterations == 2 {
                return true;
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
    fn divisibility_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 2 == 0 || prime % 3 == 0 {
            return prime < 4;
        }
        for p in (5..=super::sqrt(prime).unwrap()).step_by(6) {
            crate::trace!("{self} {} {}", p, p + 2).unwrap();
            if prime % p == 0 || prime % (p + 2) == 0 {
                return false;
            }
        }
        true
    }
}
