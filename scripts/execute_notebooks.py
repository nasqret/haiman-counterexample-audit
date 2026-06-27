#!/usr/bin/env python3
"""Execute selected notebooks in place with nbclient.

This avoids depending on a globally installed nbconvert wrapper and uses the
same Python environment as Jupyter Book.
"""

from __future__ import annotations

import argparse
from pathlib import Path

import nbformat
from nbclient import NotebookClient


def execute(path: Path, timeout: int) -> None:
    with path.open("r", encoding="utf-8") as handle:
        notebook = nbformat.read(handle, as_version=4)
    client = NotebookClient(
        notebook,
        timeout=timeout,
        kernel_name=notebook.metadata.get("kernelspec", {}).get("name", "python3"),
        resources={"metadata": {"path": str(path.parent)}},
        allow_errors=False,
    )
    client.execute()
    with path.open("w", encoding="utf-8") as handle:
        nbformat.write(notebook, handle)
    print(f"executed {path}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("notebooks", nargs="+", type=Path)
    parser.add_argument("--timeout", type=int, default=180)
    args = parser.parse_args()
    for notebook in args.notebooks:
        execute(notebook, args.timeout)


if __name__ == "__main__":
    main()

