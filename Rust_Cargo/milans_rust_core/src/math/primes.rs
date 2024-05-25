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
pub trait MillerRabinTest {
    fn miller_rabin_test(self) -> bool;
    fn miller_rabin_test_iter(self, iterations: usize) -> bool;
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
}
