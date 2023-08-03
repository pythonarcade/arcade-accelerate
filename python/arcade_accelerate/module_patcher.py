import sys

from importlib.abc import Loader, MetaPathFinder
from importlib.machinery import ModuleSpec
from types import ModuleType, SimpleNamespace
from typing import Sequence


class AutoPopulatingDictionary(dict):
    def __missing__(self, key):
        self[key] = item = SimpleNamespace()
        return item


class PatchingMetaPathFinder(MetaPathFinder):
    def __init__(self, patches):
        self._patches = patches

    def _remaining_meta_path(self):
        index_of_self = sys.meta_path.index(self)
        remaining_meta_path = sys.meta_path[index_of_self + 1 :]
        return remaining_meta_path

    def find_spec(
        self, fullname: str, path: Sequence[str] | None, target: ModuleType | None = ...
    ) -> ModuleSpec | None:
        spec = None
        for finder in self._remaining_meta_path():
            spec = finder.find_spec(fullname, path, target)
            if spec is not None:
                if spec.name in self._patches:
                    if type(spec) is not ModuleSpec:
                        raise Exception(
                            "Type of module spec to be ModuleSpec, not a subtype"
                        )
                    spec.loader = PatchingLoader(
                        spec.loader, self._patches[spec.name].__dict__
                    )
                break
        return spec


class PatchingLoader(Loader):
    def __init__(self, loader: Loader, patches: dict):
        self._loader = loader
        self._patches = patches

    def create_module(self, spec: ModuleSpec) -> ModuleType | None:
        return self._loader.create_module(spec)

    def exec_module(self, module: ModuleType) -> None:
        self._loader.exec_module(module)
        module.__dict__.update(self._patches)
