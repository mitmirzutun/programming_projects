macro_rules! pow_mod {
    ($type1:ty,$type2:ty) => {
        impl PowMod for $type1 {
            fn mul_mod(self, other: $type1, modulus: $type1) -> $type1 {
                (self as $type2 * other as $type2 % modulus as $type2) as $type1
            }
            fn pow_mod(mut self, exponent: u128, modulus: $type1) -> $type1 {
                let mut result = 1;
                for bit in 0..128 - exponent.leading_zeros() {
                    if exponent & (1u128 << bit) != 0 {
                        result = result.mul_mod(self, modulus);
                    }
                    self = self.mul_mod(self, modulus)
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
pow_mod!(u8, u16; i8, i16; u16, u32; i16, i32; u32, u64; i32, i64; u64, u128; i64, i128; usize, u128; isize, i128);
impl PowMod for u128 {
    fn mul_mod(self, other: Self, modulus: Self) -> Self {
        use num::ToPrimitive;
        use num_bigint::ToBigUint;
        (self.to_biguint().unwrap() * other.to_biguint().unwrap() % modulus.to_biguint().unwrap())
            .to_u128()
            .unwrap()
    }
    fn pow_mod(mut self, exponent: u128, modulus: Self) -> Self {
        let mut result = 1;
        for bit in 0..128 - exponent.leading_zeros() {
            if exponent & (1u128 << bit) != 0 {
                result = result.mul_mod(self, modulus);
            }
            self = self.mul_mod(self, modulus)
        }
        result
    }
}
impl PowMod for i128 {
    fn mul_mod(self, other: Self, modulus: Self) -> Self {
        use num::ToPrimitive;
        use num_bigint::ToBigInt;
        (self.to_bigint().unwrap() * other.to_bigint().unwrap() % modulus.to_bigint().unwrap())
            .to_i128()
            .unwrap()
    }
    fn pow_mod(mut self, exponent: u128, modulus: Self) -> Self {
        let mut result = 1;
        for bit in 0..128 - exponent.leading_zeros() {
            if exponent & (1u128 << bit) != 0 {
                result = result.mul_mod(self, modulus);
            }
            self = self.mul_mod(self, modulus)
        }
        result
    }
}
impl<T> PowMod for std::rc::Rc<T>
where
    T: PowMod + Clone,
{
    fn mul_mod(self, other: Self, modulus: Self) -> Self {
        std::rc::Rc::new(<T as PowMod>::mul_mod(
            (*self).clone(),
            (*other).clone(),
            (*modulus).clone(),
        ))
    }
    fn pow_mod(self, exponent: u128, modulus: Self) -> Self {
        std::rc::Rc::new(<T as PowMod>::pow_mod(
            (*self).clone(),
            exponent,
            (*modulus).clone(),
        ))
    }
}
impl<T> PowMod for std::sync::Arc<T>
where
    T: PowMod + Clone,
{
    fn mul_mod(self, other: Self, modulus: Self) -> Self {
        std::sync::Arc::new(<T as PowMod>::mul_mod(
            (*self).clone(),
            (*other).clone(),
            (*modulus).clone(),
        ))
    }
    fn pow_mod(self, exponent: u128, modulus: Self) -> Self {
        std::sync::Arc::new(<T as PowMod>::pow_mod(
            (*self).clone(),
            exponent,
            (*modulus).clone(),
        ))
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
        <T as PowMod>::mul_mod(*self, other, modulus)
    }
    fn pow_mod(&self, exponent: u128, modulus: Self) -> Self {
        <T as PowMod>::pow_mod(*self, exponent, modulus)
    }
}
macro_rules! miller_rabin_iter {
    ($prime:expr,$base:expr) => {{
        let prime = $prime;
        let base = $base;
        let exponent = prime >> 1;
        let squares = exponent.trailing_zeros();
        let exponent = (exponent >> squares) as u128;
        miller_rabin_iter!(prime, base, exponent, squares)
    }};
    ($prime:expr,$base:expr,$exponent:expr,$squares:expr) => {{
        let prime = $prime;
        let base = $base;
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
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if miller_rabin_iter!(prime, 2) {
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
        if miller_rabin_iter!(prime, 2) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for i8 {
    fn miller_rabin_test(self) -> bool {
        let prime = self;
        if prime < 2 {
            return false;
        }
        if prime % 6 != 1 && prime % 6 != 5 {
            return prime < 4;
        }
        if miller_rabin_iter!(prime, 2) {
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
        if miller_rabin_iter!(prime, 2) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for u16 {
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
        if miller_rabin_iter!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if miller_rabin_iter!(prime, 7, exponent, squares) {
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
        if miller_rabin_iter!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if miller_rabin_iter!(prime, 7, exponent, squares) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for i16 {
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
        if miller_rabin_iter!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if miller_rabin_iter!(prime, 7, exponent, squares) {
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
        if miller_rabin_iter!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if miller_rabin_iter!(prime, 7, exponent, squares) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for u32 {
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
        if miller_rabin_iter!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if miller_rabin_iter!(prime, 7, exponent, squares)
            || miller_rabin_iter!(prime, 61, exponent, squares)
        {
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
        if miller_rabin_iter!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if miller_rabin_iter!(prime, 7, exponent, squares) {
            return false;
        }
        if iterations == 2 {
            return true;
        }
        if miller_rabin_iter!(prime, 61, exponent, squares) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for i32 {
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
        if miller_rabin_iter!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if miller_rabin_iter!(prime, 7, exponent, squares)
            || miller_rabin_iter!(prime, 61, exponent, squares)
        {
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
        if miller_rabin_iter!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if miller_rabin_iter!(prime, 7, exponent, squares) {
            return false;
        }
        if iterations == 2 {
            return true;
        }
        if miller_rabin_iter!(prime, 61, exponent, squares) {
            return false;
        }
        true
    }
}
impl MillerRabinTest for u64 {
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
        if miller_rabin_iter!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as u64 {
            let prime = prime as u32;
            if miller_rabin_iter!(prime, 7, exponent, squares)
                || miller_rabin_iter!(prime, 61, exponent, squares)
            {
                return false;
            }
            return true;
        }
        for base in 3..=43 {
            if miller_rabin_iter!(prime, base, exponent, squares) {
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
        if miller_rabin_iter!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as u64 {
            let prime = prime as u32;
            if miller_rabin_iter!(prime, 7, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_iter!(prime, 61, exponent, squares) {
                return false;
            }
            return true;
        }
        for base in (3..=43).take(iterations - 1) {
            if miller_rabin_iter!(prime, base, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl MillerRabinTest for i64 {
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
        if miller_rabin_iter!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as i64 {
            let prime = prime as u32;
            if miller_rabin_iter!(prime, 7, exponent, squares)
                || miller_rabin_iter!(prime, 61, exponent, squares)
            {
                return false;
            }
            return true;
        }
        for base in 3..=43 {
            if miller_rabin_iter!(prime, base, exponent, squares) {
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
        if miller_rabin_iter!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as i64 {
            let prime = prime as u32;
            if miller_rabin_iter!(prime, 7, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_iter!(prime, 61, exponent, squares) {
                return false;
            }
            return true;
        }
        for base in (3..=43).take(iterations - 1) {
            if miller_rabin_iter!(prime, base, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl MillerRabinTest for usize {
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
        if miller_rabin_iter!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as usize {
            let prime = prime as u32;
            if miller_rabin_iter!(prime, 7, exponent, squares)
                || miller_rabin_iter!(prime, 61, exponent, squares)
            {
                return false;
            }
            return true;
        }
        for base in 3..=43 {
            if miller_rabin_iter!(prime, base, exponent, squares) {
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
        if miller_rabin_iter!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as usize {
            let prime = prime as u32;
            if miller_rabin_iter!(prime, 7, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_iter!(prime, 61, exponent, squares) {
                return false;
            }
            return true;
        }
        for base in (3..=43).take(iterations - 1) {
            if miller_rabin_iter!(prime, base, exponent, squares) {
                return false;
            }
        }
        true
    }
}
impl MillerRabinTest for isize {
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
        if miller_rabin_iter!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 {
            return true;
        }
        if prime <= u32::MAX as isize {
            let prime = prime as u32;
            if miller_rabin_iter!(prime, 7, exponent, squares)
                || miller_rabin_iter!(prime, 61, exponent, squares)
            {
                return false;
            }
            return true;
        }
        for base in 3..=43 {
            if miller_rabin_iter!(prime, base, exponent, squares) {
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
        if miller_rabin_iter!(prime, 2, exponent, squares) {
            return false;
        }
        if prime < 2047 || iterations == 1 {
            return true;
        }
        if prime <= u32::MAX as isize {
            let prime = prime as u32;
            if miller_rabin_iter!(prime, 7, exponent, squares) {
                return false;
            }
            if iterations == 2 {
                return true;
            }
            if miller_rabin_iter!(prime, 61, exponent, squares) {
                return false;
            }
            return true;
        }
        for base in (3..=43).take(iterations - 1) {
            if miller_rabin_iter!(prime, base, exponent, squares) {
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
impl<T> MillerRabinTest for std::sync::Arc<std::sync::Mutex<T>>
where
    T: MillerRabinTest + Clone,
{
    fn miller_rabin_test(self) -> bool {
        let tmp = self.lock().unwrap();
        <T as MillerRabinTest>::miller_rabin_test(tmp.clone())
    }
    fn miller_rabin_test_iter(self, iterations: usize) -> bool {
        <T as MillerRabinTest>::miller_rabin_test_iter(self.lock().unwrap().clone(), iterations)
    }
}
#[cfg(test)]
mod test {
    macro_rules! pow_mod {
        ($type1:ty) => {
            assert_eq!(
                <$type1 as super::PowMod>::mul_mod(6, 4, <$type1>::MAX),
                24,
                "Expected the 4*6 mod <{} as PowMod>::MAX to be 24",
                stringify!($type1)
            );
            assert_eq!(
                <std::rc::Rc<$type1> as super::PowMod>::mul_mod(
                    std::rc::Rc::new(6),
                    std::rc::Rc::new(4),
                    std::rc::Rc::new(<$type1>::MAX)
                ),
                std::rc::Rc::new(24),
                "Expected the 4*6 mod <std::rc::R<{}> as PowMod>::MAX to be 24",
                stringify!($type1)
            );
            assert_eq!(
                <$type1 as super::PowModRef>::mul_mod(&6, 4, <$type1>::MAX),
                24,
                "Expected the 4*6 mod <{} as PowModRef>::MAX to be 24",
                stringify!($type1)
            );
            assert_eq!(
                <$type1 as super::PowMod>::pow_mod(6, 3, 16),
                8,
                "Expected the 6**3 mod 16{} to be 8",
                stringify!($type1)
            );
            assert_eq!(
                <$type1 as super::PowMod>::mul_mod(
                    <$type1>::MAX - 1,
                    <$type1>::MAX - 1,
                    <$type1>::MAX
                ),
                1,
                "Expected the ({0}::MAX-1)**2 mod {0}::MAX to be 24",
                stringify!($type1)
            );
        };
    }
    #[test]
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
    #[test]
    fn miller_rabin_test() {
        use super::MillerRabinTest;
        assert_eq!(
            (-128..=127i8)
                .filter(|x| MillerRabinTest::miller_rabin_test(x))
                .collect::<Vec<_>>(),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127
            ]
        );
        assert_eq!(
            (-128..=127i8)
                .filter(|x| MillerRabinTest::miller_rabin_test_iter(x, 0))
                .collect::<Vec<_>>(),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 25, 29, 31, 35, 37, 41, 43, 47, 49, 53, 55, 59, 61,
                65, 67, 71, 73, 77, 79, 83, 85, 89, 91, 95, 97, 101, 103, 107, 109, 113, 115, 119,
                121, 125, 127
            ]
        );
        assert_eq!(
            (-128..=127i8)
                .filter(|x| MillerRabinTest::miller_rabin_test_iter(x, 1))
                .collect::<Vec<_>>(),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127
            ]
        );
        assert_eq!(
            (0..=255u8)
                .filter(|x| MillerRabinTest::miller_rabin_test(x))
                .collect::<Vec<_>>(),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251
            ]
        );
        assert_eq!(
            (0..=255u8)
                .filter(|x| MillerRabinTest::miller_rabin_test_iter(x, 0))
                .collect::<Vec<_>>(),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 25, 29, 31, 35, 37, 41, 43, 47, 49, 53, 55, 59, 61,
                65, 67, 71, 73, 77, 79, 83, 85, 89, 91, 95, 97, 101, 103, 107, 109, 113, 115, 119,
                121, 125, 127, 131, 133, 137, 139, 143, 145, 149, 151, 155, 157, 161, 163, 167,
                169, 173, 175, 179, 181, 185, 187, 191, 193, 197, 199, 203, 205, 209, 211, 215,
                217, 221, 223, 227, 229, 233, 235, 239, 241, 245, 247, 251, 253
            ]
        );
        assert_eq!(
            (0..=255u8)
                .filter(|x| MillerRabinTest::miller_rabin_test_iter(x, 1))
                .collect::<Vec<_>>(),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251
            ]
        );
        assert_eq!(
            (i16::MIN..=i16::MAX)
                .filter(|x| MillerRabinTest::miller_rabin_test(x))
                .count(),
            3512
        );
        assert_eq!(
            (i16::MIN..=i16::MAX)
                .filter(|x| MillerRabinTest::miller_rabin_test_iter(x, 0))
                .count(),
            10924
        );
        assert_eq!(
            (i16::MIN..=i16::MAX)
                .filter(|x| MillerRabinTest::miller_rabin_test_iter(x, 1))
                .count(),
            3519
        );
        assert_eq!(
            (i16::MIN..=i16::MAX)
                .filter(|x| MillerRabinTest::miller_rabin_test_iter(x, 2))
                .count(),
            3512
        );
        assert_eq!(
            (0..=u16::MAX)
                .filter(|x| MillerRabinTest::miller_rabin_test(x))
                .count(),
            6542
        );
        assert_eq!(
            (0..=u16::MAX)
                .filter(|x| MillerRabinTest::miller_rabin_test_iter(x, 0))
                .count(),
            21846
        );
        assert_eq!(
            (0..=u16::MAX)
                .filter(|x| MillerRabinTest::miller_rabin_test_iter(x, 1))
                .count(),
            6553
        );
        assert_eq!(
            (0..=u16::MAX)
                .filter(|x| MillerRabinTest::miller_rabin_test_iter(x, 2))
                .count(),
            6542
        );
    }
}
