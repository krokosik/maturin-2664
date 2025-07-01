use pyo3::prelude::*;

/// This module hosts Python wrappers for communicating with Bluepill Rust firmware.
#[pymodule]
fn rustpill_clients(_m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
