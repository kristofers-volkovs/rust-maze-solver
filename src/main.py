import maze

from maze_visualizer import MazeVisualizer

def main():
    n = 50
    maze_start = (1, 1)
    maze_end = (n, n)

    m = maze.Maze(n, maze_start, maze_end)
    m.solve()
    m_visualizer = MazeVisualizer(maze=m)
    m_visualizer.plot_maze()

if __name__ == '__main__':
    main()
