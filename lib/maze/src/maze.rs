use pyo3::prelude::*;

#[pyclass]
pub struct Maze {
}

#[pymethods]
impl Maze {
    #[new]
    pub fn new(n: usize) -> Maze { Maze {}}
}

/// A Python module implemented in Rust.
#[pymodule]
fn maze(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Maze>()?;
    Ok(())
}
