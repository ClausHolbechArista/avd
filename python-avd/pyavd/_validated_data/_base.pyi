# Copyright (c) 2026 Arista Networks, Inc.
# Generated from the AVD schema. Do not edit by hand.
# These stubs are the Python API documentation; there is no companion Python source.
# ruff: noqa: PYI021
from collections.abc import Iterator, Sequence
from typing import Any, Generic, TypeVar, overload

from pyavd._utils.undefined import UndefinedType

_T_co = TypeVar("_T_co", covariant=True)
_K = TypeVar("_K")

class _Model:
    """
    Immutable, unhashable model with borrowed defaults for absent fields.

    Without a schema default, absent collections expose empty views and absent scalars return
    None. Explicit null stays None and suppresses fallback. Access never inserts host data.
    Equality compares backing contents of the same nominal type, not child accessor defaults.
    """
    def __bool__(self) -> bool:
        """Whether the backing collection has keys; child accessor defaults do not populate it."""
    def __eq__(self, other: object) -> bool:
        """Compare complete contents, ignoring dictionary key order but preserving list order and nulls."""
    def _is_defined(self, name: str) -> bool:
        """
        Whether host data supplies a non-null field; defaults are not supplied values.

        False, zero, and empty collections count as defined. Unknown names raise AttributeError.
        """
    def _is_default(self, name: str) -> bool:
        """
        Whether access uses a schema default or an implicit empty collection fallback.

        Supplied values, including null or values equal to defaults, return False. An absent
        scalar without a default also returns False. Unknown names raise AttributeError.
        """

class _Collection(Generic[_T_co]):
    """Immutable, unhashable collection; equality requires the same nominal type and stored item order."""
    def __len__(self) -> int: ...
    def __iter__(self) -> Iterator[_T_co]: ...
    def __reversed__(self) -> Iterator[_T_co]: ...
    def __eq__(self, other: object) -> bool: ...
    def count(self, value: object) -> int:
        """Count items equal to value; this searches values, not primary keys."""
    def index(self, value: object, start: int | None = None, stop: int | None = None) -> int:
        """Find the first equal item position in the bounded range; raise ValueError if absent."""

class _List(_Collection[_T_co], Sequence[_T_co]):
    """Positional collection; indexing, slicing, and membership follow the Sequence contract."""

class _IndexedList(_Collection[_T_co], Generic[_K, _T_co]):
    """Key-indexed collection, not a Sequence; iteration and positional searches operate on values."""
    def __contains__(self, key: _K) -> bool: ...
    def __getitem__(self, key: _K) -> _T_co: ...
    @overload
    def get(self, key: _K) -> _T_co | UndefinedType: ...
    @overload
    def get(self, key: _K, default: Any) -> _T_co | Any: ...
    def keys(self) -> Iterator[_K]: ...
    def values(self) -> Iterator[_T_co]: ...
    def items(self) -> Iterator[tuple[_K, _T_co]]: ...

_D = TypeVar("_D")

class _DynamicEntry(Generic[_T_co]):
    """One concrete dynamic key and its typed, immutable value."""
    @property
    def key(self) -> str: ...
    @property
    def value(self) -> _T_co: ...

class _DynamicCollection(Generic[_T_co]):
    """Read-only key lookup with value iteration, grouped by one schema selector."""
    def __len__(self) -> int: ...
    def __contains__(self, key: str) -> bool: ...
    def __getitem__(self, key: str) -> _DynamicEntry[_T_co]: ...
    def __iter__(self) -> Iterator[_DynamicEntry[_T_co]]: ...
    @overload
    def get(self, key: str) -> _DynamicEntry[_T_co] | UndefinedType: ...
    @overload
    def get(self, key: str, default: _D) -> _DynamicEntry[_T_co] | _D: ...
    def keys(self) -> Iterator[str]: ...
    def values(self) -> Iterator[_DynamicEntry[_T_co]]: ...
    def items(self) -> Iterator[tuple[str, _DynamicEntry[_T_co]]]: ...
