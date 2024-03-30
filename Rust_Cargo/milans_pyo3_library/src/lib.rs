#[pyo3::prelude::pymodule]
fn milans_pyo3_library<'py>(
    _py: pyo3::Python<'py>,
    module: &'py pyo3::prelude::PyModule,
) -> pyo3::PyResult<()> {
    Ok(())
}