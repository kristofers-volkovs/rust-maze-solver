import time
import maze

from maze_visualizer import MazeVisualizer

def main():
    n = 100
    maze_start = (1, 1)
    maze_end = (n, n)

    start_time = time.time()
    m = maze.Maze(n, maze_start, maze_end)
    print(f"Maze generation time: {time.time() - start_time}")

    start_time = time.time()
    m.solve()
    print(f"Maze solving time: {time.time() - start_time}")

    # print(m.maze_solver.get_solved_path())

    m_visualizer = MazeVisualizer(maze=m)
    m_visualizer.plot_maze()

if __name__ == '__main__':
    main()
