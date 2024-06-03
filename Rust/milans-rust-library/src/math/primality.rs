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
