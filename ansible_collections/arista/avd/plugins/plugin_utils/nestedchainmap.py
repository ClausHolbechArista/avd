from collections import ChainMap
from itertools import chain


class NestedChainMap(ChainMap):
    """
    NestedChainMap is a wrapper of ChainMap, where child objects are returned as chains.

    Mappings are returned as NestedChainMap instances.

    Sequences they are returned as itertools.chain instances if kwarg liststrategy == "append" (default).

    For other object only the first match is returned (same as regular ChainMaps)
    """

    def __init__(self, *args, liststrategy="append", exempt=tuple(), **kwargs):
        self.liststrategy = liststrategy
        self.exempt = exempt
        super().__init__(*args, **kwargs)

    def __getitem__(self, key):
        if key in self.exempt:
            return super().__getitem__(key)

        values = [onemap[key] for onemap in self.maps if key in onemap]
        if not values:
            return super().__getitem__(key)

        if isinstance(values[0], dict):
            return NestedChainMap(*values, liststrategy=self.liststrategy, exempt=self.exempt)

        if self.liststrategy == "append" and isinstance(values[0], list):
            return chain(values.reverse())

        return super().__getitem__(key)
