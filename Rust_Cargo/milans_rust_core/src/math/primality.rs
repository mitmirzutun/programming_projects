macro_rules! pow_mod {
    ($type1:ty,$type2:ty) => {
        impl PowMod for $type1 {
            fn mul_mod(self, other: Self, modulus: Self) -> Self {
                (self as $type2 * other as $type2 % modulus as $type2) as $type1
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
mod test {
    use crate::math::{primality::*, *};
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
    #[test]
    fn primality() {
        pow_mod();
    }
}
