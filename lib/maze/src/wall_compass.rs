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

    pub fn position_available_paths(&self, path: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut possible_directions = vec![];

        let position = path.last().unwrap();
        let row = position.0;
        let col = position.1;

        if !self.north[row][col] {
            possible_directions.push((row, col + 1));
        }
        if !self.east[row][col] {
            possible_directions.push((row + 1, col));
        }
        if !self.south[row][col] {
            possible_directions.push((row, col - 1));
        }
        if !self.west[row][col] {
            possible_directions.push((row - 1, col));
        }

        if path.len() > 1 {
            let mut directions = vec![];
            for dir in possible_directions.iter() {
                if !path.contains(dir) {
                    directions.push(*dir);
                }
            }

            return directions;
        } else {
            return possible_directions;
        }
    }
}

impl Clone for WallCompass {
    fn clone(&self) -> Self {
        WallCompass {
            north: self.north.clone(),
            east: self.east.clone(),
            south: self.south.clone(),
            west: self.west.clone(),
        }
    }
}
