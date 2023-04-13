import time
import math
import matplotlib

matplotlib.use("TkAgg")
from matplotlib import pyplot as plt


class MazeVisualizer():

    def __init__(self, maze):
        self.maze = maze

    def plot_maze(self):
        start_time = time.time()

        _fig, ax = plt.subplots()
        m_size = self.maze.get_maze_len() + 1

        for row_idx in range(1, m_size):
            for col_idx in range(1, m_size):
                if self.maze.get_north(row_idx, col_idx):
                    x = [col_idx + 1, col_idx + 1]
                    y = [row_idx, row_idx + 1]
                    ax.plot(x, y, color='black')

                if self.maze.get_east(row_idx, col_idx):
                    x = [col_idx, col_idx + 1]
                    y = [row_idx + 1, row_idx + 1]
                    ax.plot(x, y, color='black')

                if self.maze.get_south(row_idx, col_idx):
                    x = [col_idx, col_idx]
                    y = [row_idx, row_idx + 1]
                    ax.plot(x, y, color='black')

                if self.maze.get_west(row_idx, col_idx):
                    x = [col_idx, col_idx + 1]
                    y = [row_idx, row_idx]
                    ax.plot(x, y, color='black')

        start = self.maze.start
        end = self.maze.end

        dot_size = 200 / math.log(m_size)
        ax.scatter(start[0] + 0.5, start[1] + 0.5, s=dot_size, color='r')
        ax.scatter(end[0] + 0.5, end[1] + 0.5, s=dot_size, color='b')

        solved_path = self.maze.get_solved_path()

        path_x = []
        path_y = []
        for point in solved_path:
            path_x.append(point[1] + 0.5)
            path_y.append(point[0] + 0.5)

        path_line_width = 12 / math.log(m_size)
        ax.plot(path_x, path_y, lw=path_line_width, color='g')

        print(f"Maze plot time: {time.time() - start_time}")

        plt.tight_layout()
        plt.show()
