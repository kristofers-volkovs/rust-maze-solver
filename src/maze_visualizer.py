import matplotlib

matplotlib.use("TkAgg")
from matplotlib import pyplot as plt


class MazeVisualizer():

    def __init__(self, maze):
        self.maze = maze

    def plot_maze(self):
        _fig, ax = plt.subplots()

        m_size = self.maze.n

        for row_idx in range(1, m_size + 1):
            for col_idx in range(1, m_size + 1):
                if self.maze.north[row_idx][col_idx]:
                    x = [col_idx, col_idx + 1]
                    y = [row_idx, row_idx]
                    ax.plot(x, y, color='black')

                if self.maze.east[row_idx][col_idx]:
                    x = [col_idx + 1, col_idx + 1]
                    y = [row_idx, row_idx + 1]
                    ax.plot(x, y, color='black')

                if self.maze.south[row_idx][col_idx]:
                    x = [col_idx, col_idx + 1]
                    y = [row_idx + 1, row_idx + 1]
                    ax.plot(x, y, color='black')

                if self.maze.west[row_idx][col_idx]:
                    x = [col_idx, col_idx]
                    y = [row_idx, row_idx + 1]
                    ax.plot(x, y, color='black')

        plt.tight_layout()
        plt.show()
