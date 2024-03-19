pub mod primality;
pub fn sqrt<T>(x: T) -> Option<T>
where
    T: From<u8>
        + PartialOrd
        + Clone
        + std::ops::Mul<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>,
{
    if x < 0.into() {
        return None;
    }
    if x == 0.into() {
        return Some(0.into());
    }
    if x < 4.into() {
        let mut low = 0.into();
        let mut mid: T = 1.into();
        let mut high = 2.into();
        while mid != low && mid != high {
            let tmp = mid.clone() * mid.clone();
            if tmp == x {
                return Some(mid);
            }
            if tmp < x {
                low = mid
            } else {
                high = mid;
            }
            mid = low.clone() + (high.clone() - low.clone()) / 2.into();
        }
    }
    let mut prev = (x.clone() + 1.into()) / 2.into();
    prev = (prev.clone() + x.clone() / prev.clone()) / 2.into();
    prev = (prev.clone() + x.clone() / prev.clone()) / 2.into();
    let mut result = (prev.clone() + x.clone() / prev.clone()) / 2.into();
    let mut next = (result.clone() + x.clone() / prev.clone()) / 2.into();
    while next != result && next != prev {
        prev = result;
        result = next.clone();
        next = (next.clone() + x.clone() / next.clone()) / 2.into();
    }
    Some(result)
}
