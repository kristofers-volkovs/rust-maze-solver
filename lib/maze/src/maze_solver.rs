use std::thread;

use pyo3::prelude::*;

use super::wall_compass::WallCompass;

#[pyclass]
#[derive(Clone)]
pub struct MazeSolver {
    correct_path: Option<Vec<(usize, usize)>>,
}

#[pymethods]
impl MazeSolver {
    #[new]
    pub fn new() -> Self {
        MazeSolver { correct_path: None }
    }

    pub fn solve_maze(&mut self, walls: &WallCompass, start: (usize, usize), end: (usize, usize)) {
        let init_path = vec![start];
        let correct_path = rec_solve(walls, init_path, end);

        match correct_path {
            Some(_) => { self.correct_path = correct_path },
            None => {}
        }
    }

    pub fn get_solved_path(&self) -> Vec<(usize, usize)> {
        match &self.correct_path {
            Some(path) => return path.clone(),
            None => return vec![],
        }
    }
}

fn rec_solve(
    walls: &WallCompass,
    mut path: Vec<(usize, usize)>,
    end: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let available_directions = walls.position_available_paths(path.clone());

    if available_directions.contains(&end) {
        path.push(end);
        return Some(path);
    }

    if available_directions.len() == 1 {
        path.push(available_directions[0]);
        return rec_solve(walls, path, end);
    } else if available_directions.len() > 1 {
        let mut join_tasks = vec![];

        for dir in available_directions.iter() {
            let mut possible_path = path.clone();
            possible_path.push(*dir);

            let walls_clone = walls.clone();
            let task = thread::spawn(move || rec_solve(&walls_clone, possible_path, end));
            join_tasks.push(task);
        }

        for task in join_tasks {
            if let Ok(task_res) = task.join() {
                match task_res {
                    Some(correct_path) => return Some(correct_path),
                    None => {}
                }
            }
        }
    }

    return None;
}
