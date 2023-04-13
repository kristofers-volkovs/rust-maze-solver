use pyo3::prelude::*;

use super::maze::Maze;
use super::maze_solver::MazeSolver;
use super::wall_compass::WallCompass;

#[pymodule]
fn maze(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Maze>()?;
    m.add_class::<MazeSolver>()?;
    m.add_class::<WallCompass>()?;
    Ok(())
}
