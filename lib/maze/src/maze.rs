use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Maze {
    n: usize, // dimension of maze

    #[pyo3(get, set)]
    north: Vec<Vec<bool>>, // is there a wall to north of cell i, j
    #[pyo3(get, set)]
    east: Vec<Vec<bool>>, // and so forth...
    #[pyo3(get, set)]
    south: Vec<Vec<bool>>,
    #[pyo3(get, set)]
    west: Vec<Vec<bool>>,

    #[pyo3(get, set)]
    visited: Vec<Vec<bool>>, // Use this array to register/query already visited cells
}

#[pymethods]
impl Maze {
    #[new]
    pub fn new(n: usize) -> Maze {
        let mut visited = vec![vec![false; n + 2]; n + 2];

        // initialize border cells as already visited
        for x in 0..(n + 2) as usize {
            visited[x][0] = true;
            visited[x][n + 1] = true;
        }
        for y in 0..(n + 2) as usize {
            visited[0][y] = true;
            visited[n + 1][y] = true;
        }

        let direction_vec = vec![vec![true; n + 2]; n + 2];

        let mut maze = Maze {
            n,
            north: direction_vec.to_owned(),
            east: direction_vec.to_owned(),
            south: direction_vec.to_owned(),
            west: direction_vec.to_owned(),
            visited: visited,
        };

        maze.generate(1, 1);
        maze
    }

    fn generate(&mut self, x: usize, y: usize) {
        self.visited[x][y] = true;

        // while there is an unvisited neighbor
        while !self.visited[x][y + 1]
            || !self.visited[x + 1][y]
            || !self.visited[x][y - 1]
            || !self.visited[x - 1][y]
        {
            // pick random neighbor
            loop {
                let r = rand::random::<f32>();
                if r < 0.25 && !self.visited[x + 1][y] {
                    self.south[x][y] = false;
                    self.north[x + 1][y] = false;
                    self.generate(x + 1, y);
                    break;
                } else if r >= 0.25 && r < 0.50 && !self.visited[x][y + 1] {
                    self.east[x][y] = false;
                    self.west[x][y + 1] = false;
                    self.generate(x, y + 1);
                    break;
                } else if r >= 0.5 && r < 0.75 && !self.visited[x - 1][y] {
                    self.north[x][y] = false;
                    self.south[x - 1][y] = false;
                    self.generate(x - 1, y);
                    break;
                } else if r >= 0.75 && r < 1.00 && !self.visited[x][y - 1] {
                    self.west[x][y] = false;
                    self.east[x][y - 1] = false;
                    self.generate(x, y - 1);
                    break;
                }
            }
        }
    }
}
