#!/usr/bin/env python3
"""Run cargo nextest scoped to crates (transitively) impacted by the diff
between HEAD and BASE_REF. Falls back to the full suite when changes touch
files outside the safe selection scope.

Usage:
    BASE_REF=origin/main scripts/test_changed.py
    DRY_RUN=1 BASE_REF=origin/main scripts/test_changed.py   # print only
"""
from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path

SAFETY_NET_EXACT = {
    "stwo_cairo_prover/Cargo.lock",
    "stwo_cairo_prover/rust-toolchain.toml",
    "stwo_cairo_prover/Cargo.toml",
    "stwo_cairo_prover/.config/nextest.toml",
}

SAFETY_NET_PREFIXES = (
    "stwo_cairo_prover/scripts/",
    "stwo_cairo_prover/test_data/",
    ".github/workflows/",
)

SLOW_TESTS_CMD = [
    "cargo", "nextest", "run",
    "--cargo-profile", "witness-opt-1",
    "--features=slow-tests",
    "-j", "1",
    "-P", "ci",
]

EXTRACT_MEM_TRACE_CMD = [
    "cargo", "nextest", "run",
    "--cargo-profile", "witness-opt-1",
    "--features=extract-mem-trace",
    "test_serialize_deserialize_mem_trace",
    "-j", "1",
    "-P", "ci",
]

FULL_NEXTEST_CMDS = [SLOW_TESTS_CMD, EXTRACT_MEM_TRACE_CMD]


def log(msg: str) -> None:
    print(f"test_changed.py: {msg}", file=sys.stderr)


def run_cmds(cmds: list[list[str]], dry_run: bool, cwd: Path) -> None:
    for cmd in cmds:
        print("+ " + " ".join(cmd), file=sys.stderr)
        if not dry_run:
            subprocess.run(cmd, check=True, cwd=cwd)


def run_full_suite(reason: str, dry_run: bool, prover_dir: Path) -> None:
    log(f"running full nextest suite (reason: {reason}).")
    run_cmds(FULL_NEXTEST_CMDS, dry_run, prover_dir)


def run_filtered_suite(filter_expr: str, dry_run: bool, prover_dir: Path) -> None:
    log(f"filter expression: {filter_expr}")
    cmds = [cmd + ["-E", filter_expr] for cmd in FULL_NEXTEST_CMDS]
    run_cmds(cmds, dry_run, prover_dir)


def workspace_packages(prover_dir: Path, repo_root: Path) -> list[tuple[str, str]]:
    """Return [(package_dir_relative_to_repo_root_with_trailing_slash, pkg_name), ...]
    sorted longest-prefix-first so the deepest-matching package wins."""
    metadata_json = subprocess.check_output(
        ["cargo", "metadata", "--no-deps", "--format-version=1"],
        text=True,
        cwd=prover_dir,
    )
    metadata = json.loads(metadata_json)
    pkg_dirs: list[tuple[str, str]] = []
    for pkg in metadata["packages"]:
        manifest = Path(pkg["manifest_path"])
        try:
            rel_dir = manifest.parent.relative_to(repo_root)
        except ValueError:
            continue
        pkg_dirs.append((f"{rel_dir.as_posix()}/", pkg["name"]))
    pkg_dirs.sort(key=lambda x: -len(x[0]))
    return pkg_dirs


def main() -> None:
    base_ref = os.environ.get("BASE_REF", "origin/main")
    dry_run = bool(os.environ.get("DRY_RUN"))

    repo_root = Path(
        subprocess.check_output(
            ["git", "rev-parse", "--show-toplevel"], text=True
        ).strip()
    )
    prover_dir = repo_root / "stwo_cairo_prover"

    try:
        base = subprocess.check_output(
            ["git", "merge-base", "HEAD", base_ref],
            text=True,
            stderr=subprocess.DEVNULL,
            cwd=repo_root,
        ).strip()
    except subprocess.CalledProcessError:
        run_full_suite(
            f"cannot resolve merge-base against {base_ref}", dry_run, prover_dir
        )
        return

    changed = [
        line
        for line in subprocess.check_output(
            ["git", "diff", "--name-only", base, "HEAD"],
            text=True,
            cwd=repo_root,
        ).splitlines()
        if line
    ]

    if not changed:
        log(f"no changes vs {base_ref}; nothing to run.")
        return

    for f in changed:
        if f in SAFETY_NET_EXACT or any(f.startswith(p) for p in SAFETY_NET_PREFIXES):
            run_full_suite(f"safety-net: {f}", dry_run, prover_dir)
            return

    pkg_dirs = workspace_packages(prover_dir, repo_root)
    pkgs: set[str] = set()
    for f in changed:
        if not f.startswith("stwo_cairo_prover/"):
            continue
        for dir_prefix, name in pkg_dirs:
            if f.startswith(dir_prefix):
                pkgs.add(name)
                break

    if not pkgs:
        log("no Rust crate impact detected; nothing to run.")
        return

    filter_expr = " + ".join(f"rdeps({n})" for n in sorted(pkgs))
    run_filtered_suite(filter_expr, dry_run, prover_dir)


if __name__ == "__main__":
    main()
