pub fn main() {
    println!("Hello, world!");
}
#[pyo3::prelude::pymodule]
fn milans_pyo3_library<'py>(
    _py: pyo3::Python<'py>,
    module: &'py pyo3::prelude::PyModule,
) -> pyo3::PyResult<()> {
    module.add_function(pyo3::wrap_pyfunction!(divisibility_test, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(miller_rabin_test, module)?)?;
    Ok(())
}
#[pyo3::prelude::pyfunction]
fn divisibility_test(prime: u64) -> bool {
    use milans_rust_core::math::primality::PrimalityTests;
    prime.divisibility_test()
}
#[pyo3::prelude::pyfunction]
#[pyo3(signature=(prime,iterations=None))]
fn miller_rabin_test(prime: u64, iterations: Option<usize>) -> bool {
    use milans_rust_core::math::primality::PrimalityTests;
    match iterations {
        None => prime.miller_rabin_test(),
        Some(iterations) => prime.miller_rabin_test_iter(iterations),
    }
}
