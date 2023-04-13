import time
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

        print(f"Maze plot time: {time.time() - start_time}")

        plt.tight_layout()
        plt.show()
