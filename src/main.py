import maze

from maze_visualizer import MazeVisualizer

def main():
    m = maze.Maze(50)

    # # Prints visited cells
    # print('visited')
    # for row in m.visited:
    #     print(row)

    m_solver = maze.MazeSolver(m)

    m_visualizer = MazeVisualizer(maze=m)

    m_visualizer.plot_maze()

if __name__ == '__main__':
    main()
