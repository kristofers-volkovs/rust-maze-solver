use pyo3::prelude::*;

#[pyclass]
pub struct WallCompass {
    pub north: Vec<Vec<bool>>, // is there a wall to north of cell i, j
    pub east: Vec<Vec<bool>>,  // and so forth...
    pub south: Vec<Vec<bool>>,
    pub west: Vec<Vec<bool>>,
}

#[pymethods]
impl WallCompass {
    #[new]
    pub fn new(n: usize) -> Self {
        let compass_vec = vec![vec![true; n + 2]; n + 2];

        WallCompass {
            north: compass_vec.clone(),
            east: compass_vec.clone(),
            south: compass_vec.clone(),
            west: compass_vec.clone(),
        }
    }

}
