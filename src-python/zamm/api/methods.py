"""Enable programmatic access to API metadata."""

from __future__ import annotations

from typing import Any, Callable, Generic, Protocol, TypeVar

T = TypeVar("T", bound="ApiArgs", covariant=True)
U = TypeVar("U")


class ApiArgs(Protocol[T]):
    """Stub for API arguments generated by quicktype."""

    @staticmethod
    def from_dict(obj: Any) -> T:
        """Create an instance of this class from a dict."""
        ...

    def to_dict(self) -> dict:
        """Return a JSON-serializable dict representation of this class."""
        ...


class ApiMethod(Generic[T, U]):
    """Models a single API method."""

    args_type: type[T]
    response_type: type[U]
    invoke: Callable[[T], U]

    def __init__(
        self, args_type: type[T], response_type: type[U], invocation: Callable[[T], U]
    ) -> None:
        """Define a new API method and its expected types."""
        self.args_type = args_type
        self.response_type = response_type
        self.invoke = invocation
