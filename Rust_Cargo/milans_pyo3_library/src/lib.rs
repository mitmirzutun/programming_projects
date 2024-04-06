#[pyo3::prelude::pymodule]
fn milans_pyo3_library<'py>(
    _py: pyo3::Python<'py>,
    module: &'py pyo3::prelude::PyModule,
) -> pyo3::PyResult<()> {
    module.add_function(pyo3::wrap_pyfunction!(divisibility_test, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(miller_rabin_test, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(prime_divisors, module)?)?;
    Ok(())
}
#[pyo3::prelude::pyfunction]
fn divisibility_test(prime: num_bigint::BigInt) -> bool {
    use num::{integer::Roots, ToPrimitive};
    use num_bigint::ToBigInt;
    if prime < 2.to_bigint().unwrap() {
        return false;
    }
    if prime.clone() % 6.to_bigint().unwrap() != 1.to_bigint().unwrap()
        && prime.clone() % 6.to_bigint().unwrap() != 5.to_bigint().unwrap()
    {
        return prime < 4.to_bigint().unwrap();
    }
    if prime.clone() % 5.to_bigint().unwrap() == 0.to_bigint().unwrap() {
        return prime == 5.to_bigint().unwrap();
    }
    if prime < u32::MAX.to_bigint().unwrap() {
        let prime = prime.to_u32().unwrap();
        for p in (7..=prime.sqrt()).step_by(30) {
            if prime % p == 0
                || prime % (p + 4) == 0
                || prime % (p + 6) == 0
                || prime % (p + 10) == 0
                || prime % (p + 12) == 0
                || prime % (p + 16) == 0
                || prime % (p + 22) == 0
                || prime % (p + 24) == 0
            {
                return false;
            }
        }
        return true;
    }
    if prime < u64::MAX.to_bigint().unwrap() {
        let prime = prime.to_u64().unwrap();
        for p in (7..=prime.sqrt()).step_by(30) {
            if prime % p == 0
                || prime % (p + 4) == 0
                || prime % (p + 6) == 0
                || prime % (p + 10) == 0
                || prime % (p + 12) == 0
                || prime % (p + 16) == 0
                || prime % (p + 22) == 0
                || prime % (p + 24) == 0
            {
                return false;
            }
        }
        return true;
    }
    true
}
#[pyo3::prelude::pyfunction]
fn prime_divisors(number: num_bigint::BigInt) -> pyo3::PyResult<Vec<num_bigint::BigUint>> {
    use num::{integer::Roots, ToPrimitive};
    use num_bigint::{ToBigInt, ToBigUint};
    if number < 1.to_bigint().unwrap() {
        return Ok(vec![]);
    }
    if number < u32::MAX.to_bigint().unwrap() {
        let mut number = number.to_u32().unwrap();
        let mut result = vec![];
        while number % 2 == 0 {
            number /= 2;
            result.push(2.to_biguint().unwrap());
        }
        while number % 3 == 0 {
            number /= 3;
            result.push(3.to_biguint().unwrap());
        }
        while number % 5 == 0 {
            number /= 5;
            result.push(5.to_biguint().unwrap());
        }
        let mut prime = 7;
        let mut psqrt = number.sqrt();
        while prime <= psqrt {
            let tmp = prime;
            while number % tmp == 0 {
                number /= tmp;
                result.push(tmp.to_biguint().unwrap());
            }
            let tmp = prime + 4;
            while number % tmp == 0 {
                number /= tmp;
                result.push(tmp.to_biguint().unwrap());
            }
            let tmp = prime + 6;
            while number % tmp == 0 {
                number /= tmp;
                result.push(tmp.to_biguint().unwrap());
            }
            let tmp = prime + 10;
            while number % tmp == 0 {
                number /= tmp;
                result.push(tmp.to_biguint().unwrap());
            }
            let tmp = prime + 12;
            while number % tmp == 0 {
                number /= tmp;
                result.push(tmp.to_biguint().unwrap());
            }
            let tmp = prime + 16;
            while number % tmp == 0 {
                number /= tmp;
                result.push(tmp.to_biguint().unwrap());
            }
            let tmp = prime + 22;
            while number % tmp == 0 {
                number /= tmp;
                result.push(tmp.to_biguint().unwrap());
            }
            let tmp = prime + 24;
            while number % tmp == 0 {
                number /= tmp;
                result.push(tmp.to_biguint().unwrap());
            }
            prime += 30;
            psqrt = number.sqrt();
        }
        if number > 1 {
            result.push(number.to_biguint().unwrap())
        }
        return Ok(result);
    }
    if number < u64::MAX.to_bigint().unwrap() {
        let mut number = number.to_u64().unwrap();
        let mut result = vec![];
        while number % 2 == 0 {
            number /= 2;
            result.push(2.to_biguint().unwrap());
        }
        while number % 3 == 0 {
            number /= 3;
            result.push(3.to_biguint().unwrap());
        }
        while number % 5 == 0 {
            number /= 5;
            result.push(5.to_biguint().unwrap());
        }
        let mut prime = 7;
        let mut psqrt = number.sqrt();
        while prime <= psqrt {
            let tmp = prime;
            while number % tmp == 0 {
                number /= tmp;
                result.push(tmp.to_biguint().unwrap());
            }
            let tmp = prime + 4;
            while number % tmp == 0 {
                number /= tmp;
                result.push(tmp.to_biguint().unwrap());
            }
            let tmp = prime + 6;
            while number % tmp == 0 {
                number /= tmp;
                result.push(tmp.to_biguint().unwrap());
            }
            let tmp = prime + 10;
            while number % tmp == 0 {
                number /= tmp;
                result.push(tmp.to_biguint().unwrap());
            }
            let tmp = prime + 12;
            while number % tmp == 0 {
                number /= tmp;
                result.push(tmp.to_biguint().unwrap());
            }
            let tmp = prime + 16;
            while number % tmp == 0 {
                number /= tmp;
                result.push(tmp.to_biguint().unwrap());
            }
            let tmp = prime + 22;
            while number % tmp == 0 {
                number /= tmp;
                result.push(tmp.to_biguint().unwrap());
            }
            let tmp = prime + 24;
            while number % tmp == 0 {
                number /= tmp;
                result.push(tmp.to_biguint().unwrap());
            }
            prime += 30;
            psqrt = number.sqrt();
        }
        if number > 1 {
            result.push(number.to_biguint().unwrap())
        }
        return Ok(result);
    }
    let mut number = number.to_biguint().unwrap();
    let mut result = vec![];
    for _ in 0..number.trailing_zeros().unwrap() {
        number >>= 1;
        result.push(2.to_biguint().unwrap())
    }
    while number.clone() % 3.to_biguint().unwrap() == 0.to_biguint().unwrap() {
        number /= 3.to_biguint().unwrap();
        result.push(3.to_biguint().unwrap());
    }
    while number.clone() % 5.to_biguint().unwrap() == 0.to_biguint().unwrap() {
        number /= 5.to_biguint().unwrap();
        result.push(5.to_biguint().unwrap());
    }
    let mut prime = 7.to_biguint().unwrap();
    let mut psqrt = number.sqrt();
    while prime <= psqrt {
        let tmp = prime.clone();
        while number.clone() % tmp.clone() == 0.to_biguint().unwrap() {
            number /= tmp.clone();
            result.push(tmp.to_biguint().unwrap());
        }
        let tmp = prime.clone() + 4.to_biguint().unwrap();
        while number.clone() % tmp.clone() == 0.to_biguint().unwrap() {
            number /= tmp.clone();
            result.push(tmp.to_biguint().unwrap());
        }
        let tmp = prime.clone() + 6.to_biguint().unwrap();
        while number.clone() % tmp.clone() == 0.to_biguint().unwrap() {
            number /= tmp.clone();
            result.push(tmp.to_biguint().unwrap());
        }
        let tmp = prime.clone() + 10.to_biguint().unwrap();
        while number.clone() % tmp.clone() == 0.to_biguint().unwrap() {
            number /= tmp.clone();
            result.push(tmp.to_biguint().unwrap());
        }
        let tmp = prime.clone() + 12.to_biguint().unwrap();
        while number.clone() % tmp.clone() == 0.to_biguint().unwrap() {
            number /= tmp.clone();
            result.push(tmp.to_biguint().unwrap());
        }
        let tmp = prime.clone() + 16.to_biguint().unwrap();
        while number.clone() % tmp.clone() == 0.to_biguint().unwrap() {
            number /= tmp.clone();
            result.push(tmp.to_biguint().unwrap());
        }
        let tmp = prime.clone() + 22.to_biguint().unwrap();
        while number.clone() % tmp.clone() == 0.to_biguint().unwrap() {
            number /= tmp.clone();
            result.push(tmp.to_biguint().unwrap());
        }
        let tmp = prime.clone() + 24.to_biguint().unwrap();
        while number.clone() % tmp.clone() == 0.to_biguint().unwrap() {
            number /= tmp.clone();
            result.push(tmp.to_biguint().unwrap());
        }
        prime += 30.to_biguint().unwrap();
        psqrt = number.sqrt();
        if unsafe { pyo3::ffi::PyErr_CheckSignals() } != 0 {
            return Err(pyo3::exceptions::PyKeyboardInterrupt::new_err(()));
        }
    }
    if number > 1.to_biguint().unwrap() {
        result.push(number.to_biguint().unwrap())
    }
    Ok(result)
}
#[pyo3::prelude::pyfunction]
fn miller_rabin_test(prime: num_bigint::BigInt) -> bool {
    use milans_rust_core::math::primality::MillerRabinTest;
    use num::ToPrimitive;
    use num_bigint::ToBigInt;
    if prime < 2.to_bigint().unwrap() {
        return false;
    }
    if prime < u32::MAX.to_bigint().unwrap() {
        prime.to_u32().unwrap().test()
    } else if prime < u64::MAX.to_bigint().unwrap() {
        prime.to_u64().unwrap().test()
    } else if prime < u128::MAX.to_bigint().unwrap() {
        prime.to_u128().unwrap().test()
    } else {
        true
    }
}
