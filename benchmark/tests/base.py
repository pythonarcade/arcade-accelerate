import sys
from pathlib import Path
from typing import Tuple

import arcade

from benchmark.timing import PerformanceTiming


class PerfTest:
    name = "default"
    type = "default"
    series_name = "default"
    instances = []

    def __init__(
        self,
        size: Tuple[int, int],
        title: str = "Perf Test",
        start_count: int = 0,
        increment_count: int = 100,
        duration: float = 60.0,
        **kwargs,
    ):
        self.size = size
        self.title = title
        self.start_count = start_count
        self.increment_count = increment_count
        self.duration = duration
        self.frame = 0
        self.timing = None

    @property
    def instance_name(self) -> str:
        return f"{self.type}_{self.name}"

    def on_draw(self):
        pass

    def on_update(self, delta_time: float):
        self.frame += 1

    def update_state(self):
        pass

    def run(self, session_dir: Path):
        self.frame = 0
        out_path = session_dir / "data"
        out_path.mkdir(parents=True, exist_ok=True)

        self.timing = PerformanceTiming(
            out_path / f"{self.instance_name}.csv",
            start_n=self.start_count,
            increment_n=self.increment_count,
            end_time=self.duration,
        )


class ArcadePerfTest(PerfTest):
    type = "arcade"

    def __init__(
        self,
        size: Tuple[int, int],
        title: str = "Perf Test",
        start_count: int = 0,
        increment_count: int = 100,
        duration: float = 60.0,
        **kwargs,
    ):
        super().__init__(
            size=size,
            title=title,
            start_count=start_count,
            increment_count=increment_count,
            duration=duration,
            **kwargs,
        )
        self.window = None

    def on_draw(self):
        pass

    def on_update(self, delta_time: float):
        return super().on_update(delta_time)

    def update_state(self):
        pass

    def run_test(self):
        """Run the test without collecting data"""
        super().run()
        self.create_window()
        self.setup()
        while not self.timing.end_run():
            self.window.dispatch_events()
            self.on_update(1 / 60)
            self.on_draw()
            self.update_state()
            self.window.flip()

    def run(self, session_dir: Path):
        """Run the test collecting data."""
        super().run(session_dir)
        self.create_window()
        self.setup()

        # last_time = time.time()
        # current_time = time.time()

        while not self.timing.end_run():
            self.window.dispatch_events()

            self.timing.start_timer("update")
            self.on_update(1 / 60)
            self.timing.stop_timer("update")

            self.window.clear()

            self.timing.start_timer("draw")
            self.on_draw()
            self.window.ctx.flush()  # Wait for draw to finish
            self.timing.stop_timer("draw")

            self.update_state()

            self.window.flip()

        self.timing.write()

    def create_window(self):
        try:
            self.window = arcade.get_window()
            self.window.set_size(*self.size)
        except RuntimeError:
            self.window = arcade.open_window(*self.size, self.title)
            # Run a few fames to warm up the window
            for _ in range(10):
                self.window.clear()
                self.window.flip()
                self.window.flip()


class AcceleratedPerfTest(ArcadePerfTest):
    type = "arcade-accelerate"

    def run(self, session_dir: Path):
        # This is necessary to unload arcade and ensure that we have the arcade-accelerate bootstrap applied
        # The test module itself is responsbile for applying the bootstrap, but arcade needs to be fully unloaded before then
        to_uncache = []
        for mod in sys.modules:
            if mod.startswith("arcade."):
                to_uncache.append(mod)

        for mod in to_uncache:
            del sys.modules[mod]

        import arcade_accelerate

        arcade_accelerate.bootstrap()
        import arcade

        super().run(session_dir)
