use pyo3::prelude::*;

use super::maze_solver::MazeSolver;
use super::wall_compass::WallCompass;

#[pyclass]
#[derive(Clone)]
pub struct Maze {
    n: usize, // dimension of maze

    wall_compass: WallCompass,
    maze_solver: MazeSolver,

    start: (usize, usize),
    end: (usize, usize),

    visited: Vec<Vec<bool>>, // Use this array to register/query already visited cells
}

#[pymethods]
impl Maze {
    #[new]
    pub fn new(n: usize, start: (usize, usize), end: (usize, usize)) -> Maze {
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

        let mut maze = Maze {
            n,
            wall_compass: WallCompass::new(n),
            maze_solver: MazeSolver::new(),
            start,
            end,
            visited,
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
                if r < 0.25 && !self.visited[x][y + 1] {
                    self.wall_compass.south[x][y + 1] = false;
                    self.wall_compass.north[x][y] = false;
                    self.generate(x, y + 1);
                    break;
                } else if r >= 0.25 && r < 0.50 && !self.visited[x + 1][y] {
                    self.wall_compass.east[x][y] = false;
                    self.wall_compass.west[x + 1][y] = false;
                    self.generate(x + 1, y);
                    break;
                } else if r >= 0.5 && r < 0.75 && !self.visited[x][y - 1] {
                    self.wall_compass.north[x][y - 1] = false;
                    self.wall_compass.south[x][y] = false;
                    self.generate(x, y - 1);
                    break;
                } else if r >= 0.75 && r < 1.00 && !self.visited[x - 1][y] {
                    self.wall_compass.west[x][y] = false;
                    self.wall_compass.east[x - 1][y] = false;
                    self.generate(x - 1, y);
                    break;
                }
            }
        }
    }

    pub fn solve(&mut self) {
        self.maze_solver
            .solve_maze(&self.wall_compass, self.start, self.end);
    }

    pub fn get_north(&self, row: usize, col: usize) -> bool {
        self.wall_compass.north[row][col]
    }

    pub fn get_east(&self, row: usize, col: usize) -> bool {
        self.wall_compass.east[row][col]
    }

    pub fn get_south(&self, row: usize, col: usize) -> bool {
        self.wall_compass.south[row][col]
    }

    pub fn get_west(&self, row: usize, col: usize) -> bool {
        self.wall_compass.west[row][col]
    }

    pub fn get_maze_len(&self) -> usize {
        self.n
    }

    pub fn get_start(&self) -> (usize, usize) {
        self.start
    }

    pub fn get_end(&self) -> (usize, usize) {
        self.end
    }

    pub fn get_solved_path(&self) -> Vec<(usize, usize)> {
        self.maze_solver.get_solved_path()
    }
}
