"""Library code for image tools."""
from dataclasses import dataclass, field
from typing import List, Optional


@dataclass(frozen=True)
class Command:
    """Command line program including arguments and optional standard input."""

    args: List[str] = field(default_factory=list)
    stdin: Optional[str] = field(default=None)

    @property
    def input(self) -> Optional[bytes]:
        """stdin input as UTF8 bytes."""
        if self.stdin:
            return self.stdin.encode("utf-8")
        else:
            return None

    def __str__(self) -> str:
        if self.stdin:
            return f"{' '.join(self.args)} <<<EOF\n{self.stdin}\nEOF;"
        else:
            return " ".join(self.args)
