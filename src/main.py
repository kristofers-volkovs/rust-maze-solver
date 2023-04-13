import maze

from maze_visualizer import MazeVisualizer

def main():
    n = 50
    m = maze.Maze(n)

    m_visualizer = MazeVisualizer(maze=m)
    m_visualizer.plot_maze()

if __name__ == '__main__':
    main()
