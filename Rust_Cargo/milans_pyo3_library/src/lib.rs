pub fn main() {
    println!("Hello, world!");
}
#[pyo3::prelude::pymodule]
fn milans_pyo3_library<'py>(
    _py: pyo3::Python<'py>,
    module: &'py pyo3::prelude::PyModule,
) -> pyo3::PyResult<()> {
    module.add_class::<PrimeGeneratorWrapper>()?;
    Ok(())
}
#[pyo3::prelude::pyclass]
#[pyo3(name = "PrimeGenerator")]
pub struct PrimeGeneratorWrapper {
    generator: milans_rust_core::math::primality::PrimeGenerator<u8>,
}
