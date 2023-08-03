import importlib
import pkgutil
from typing import List, Optional, Type

from benchmark import OUT_DIR
from benchmark.graph import DataSeries, PerfGraph
from benchmark.tests.base import PerfTest


def find_test_classes(path: str) -> List[Type[PerfTest]]:
    """Find all test classes in submodules"""
    target_module = importlib.import_module(f"benchmark.tests.{path}")

    classes = []
    for v in pkgutil.iter_modules(target_module.__path__):
        module = importlib.import_module(f"benchmark.tests.{path}.{v.name}")
        if hasattr(module, "Test"):
            classes.append(module.Test)
        else:
            print(
                (
                    "WARNING: "
                    f"Module '{module.__name__}' does not have a Test class. "
                    "Please add a test class or rename the class to 'Test'."
                )
            )

    return classes


class TestManager:
    """
    Finds and executes tests

    :param str session: The session name.
    :param bool debug: If True, print debug messages.
    """

    def __init__(self, session: str, debug: bool = True):
        self.debug = debug
        self.session = session
        self.session_dir = OUT_DIR / session
        self.session_dir.mkdir(parents=True, exist_ok=True)
        self.data_dir = self.session_dir / "data"

        self.test_classes: List[Type[PerfTest]] = []
        self.test_instances: List[PerfTest] = []

    @property
    def num_test_classes(self) -> int:
        return len(self.test_classes)

    @property
    def num_test_instances(self) -> int:
        return len(self.test_instances)

    def find_test_classes(
        self,
        type: Optional[str] = None,
        name: Optional[str] = None,
    ):
        """
        Find test classes based on type and name.

        :param str type: The type of test to run.
        :param str name: The name of the test to run.
        :return: The number of test classes found.
        """
        all_classes = find_test_classes("arcade")
        all_classes += find_test_classes("arcade_accelerate")

        for cls in all_classes:
            if type is not None and cls.type != type:
                continue
            if name is not None and cls.name != name:
                continue
            self.test_classes.append(cls)

        if self.debug:
            num_classes = len(self.test_classes)
            print(f"Found {num_classes} test classes")
            for cls in self.test_classes:
                print(f" -> {cls.type}.{cls.name}")

    def create_test_instances(self):
        """
        Create test instances based on each test's instances attribute.
        """
        for cls in self.test_classes:
            # If a test have multiple instances, create one instance for each
            if cls.instances:
                for params, _ in cls.instances:
                    self.add_test_instance(cls(self.session_dir, **params))
            else:
                self.add_test_instance(cls(self.session_dir))

        if self.debug:
            num_instances = len(self.test_instances)
            print(f"Created {num_instances} test instances")
            for instance in self.test_instances:
                print(f" -> {instance.type}.{instance.name}")

    def add_test_instance(self, instance: PerfTest):
        """Validate instance"""
        if instance.name == "default":
            raise ValueError(
                (
                    "Test name cannot be 'default'."
                    "Please add a class attribute 'name' to your test class."
                    f"Class: {instance}"
                )
            )
        self.test_instances.append(instance)

    def get_test_instance(self, name: str) -> Optional[PerfTest]:
        for instance in self.test_instances:
            if instance.instance_name == name:
                return instance

    def run(self):
        """Run all tests"""
        for instance in self.test_instances:
            instance.start()
            instance.join()

    def create_graph(
        self,
        file_name: str,
        title: str,
        x_label: str,
        y_label: str,
        series_names=[],
    ):
        """Create a graph using matplotlib"""
        print("Creating graph : {title}} [{x_label}, {y_label}]}]")
        series = []
        skip = False
        for _series in series_names:
            # Check if we have a test instance with this name
            instance = self.get_test_instance(_series)
            if instance is None:
                print(f" -> No test instance found for series '{_series}'")
                skip = True

            path = self.data_dir / f"{_series}.csv"
            if not path.exists():
                print(
                    f"No data found for series '{_series}' in session '{self.session}'"
                )
                skip = True

            if skip:
                continue

            series.append(DataSeries(instance.name, path))

        out_path = self.session_dir / "graphs"
        out_path.mkdir(parents=True, exist_ok=True)
        out_path = out_path / f"{file_name}.png"
        graph = PerfGraph(title, x_label, y_label, series)
        graph.create(out_path)
