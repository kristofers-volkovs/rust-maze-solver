use pyo3::prelude::*;

use super::maze::Maze;

#[pyclass]
pub struct MazeSolver {
    maze: Maze,
}

#[pymethods]
impl MazeSolver {
    #[new]
    pub fn new(maze: Maze) -> MazeSolver {
        MazeSolver { maze }
    }

    pub fn solve_maze(&self, x: i32, y: i32) {}
}
