pub mod primality;
#[macro_export]
macro_rules! gcd {
    ($number:expr) => {
        $number
    };
    ($a:expr,$b:expr) => {{
        let _a = $a.clone();
        let _b = $b.clone();
        let mut a = _a.clone();
        let mut b = _b.clone();
        while b != 0 {
            let tmp = a % b;
            a = b;
            b = tmp;
        }
        a
    }};
}
#[macro_export]
macro_rules! lcm {
    ($number:expr)=>{$number};
    ($a:expr,$b:expr) => {
        {let a=$a.clone();let b=$b.clone();a*(b/$crate::gcd!(a,b))}
    };
    ($a:expr,$($b:expr,$c:expr),+) => {$crate::lcm!($a,$($crate::lcm!($b,$c)),+)};
    ($a:expr,$b:expr,$($c:expr,$d:expr),+) => {$crate::lcm!($crate::lcm!($a,$b),$($crate::lcm!($c,$d)),+)};
}
