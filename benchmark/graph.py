import csv
from pathlib import Path

import matplotlib.pyplot as plt
import seaborn as sns

sns.set_style("whitegrid")

FPS = 1
SPRITE_COUNT = 2
DRAWING_TIME = 3
PROCESSING_TIME = 4


class DataSeries:
    def __init__(self, name: str, path: Path) -> None:
        self.name = name
        self.path = path
        # Data
        self.count = []
        self.processing_time = []
        self.draw_time = []
        self.fps = []
        # Process data
        self._process_data()

    def _process_data(self):
        rows = self._read_file(self.path)
        for row in rows:
            self.count.append(row[SPRITE_COUNT])
            self.fps.append(row[FPS])
            self.processing_time.append(row[PROCESSING_TIME])
            self.draw_time.append(row[DRAWING_TIME])

    def _read_file(self, path: Path):
        results = []
        with open(path) as csv_file:
            csv_reader = csv.reader(csv_file, delimiter=",")
            first_row = True
            for row in csv_reader:
                if first_row:
                    first_row = False
                else:
                    results.append([float(cell) for cell in row])

            return results


class PerfGraph:
    def __init__(self, title: str, label_x: str, label_y: str) -> None:
        self.title = title
        self.label_x = label_x
        self.label_y = label_y
        self.series = []

    def add_series(self, series: DataSeries):
        self.series.append(series)

    def create(self, output_path: Path):
        plt.title(self.title)

        for series in self.series:
            plt.plot(series.count, series.processing_time, label=series.name)

        plt.legend(loc="upper left", shadow=True, fontsize="large")
        plt.xlabel(self.label_x)
        plt.ylabel(self.label_y)

        plt.savefig(output_path)
        plt.clf()


if __name__ == "__main__":
    from benchmark import OUT_DIR

    OUTPUT_ROOT = OUT_DIR / "test" / "graphs"
    OUTPUT_ROOT.mkdir(parents=True, exist_ok=True)
    path = OUT_DIR / "test" / "data"

    graph = PerfGraph(
        "Time To Detect Collisions", label_x="Sprite Count", label_y="Time"
    )
    graph.add_series(DataSeries("Arcade 0", path / "arcade_collision-0.csv"))
    graph.add_series(DataSeries("Arcade 1", path / "arcade_collision-1.csv"))
    graph.add_series(DataSeries("Arcade 2", path / "arcade_collision-2.csv"))
    graph.add_series(DataSeries("Arcade 3", path / "arcade_collision-3.csv"))
    graph.create(OUTPUT_ROOT / "arcade_collision.png")
