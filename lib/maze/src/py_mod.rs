use pyo3::prelude::*;

use super::maze::Maze;

#[pymodule]
fn maze(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Maze>()?;
    Ok(())
}
