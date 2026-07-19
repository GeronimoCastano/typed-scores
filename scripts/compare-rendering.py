#!/usr/bin/env python3
"""Compare two sets of binary PPM pages with a small raster tolerance."""

from __future__ import annotations

import argparse
from pathlib import Path


def read_token(stream) -> bytes:
    while True:
        token = stream.readline()
        if not token:
            raise ValueError("unexpected end of PPM header")
        token = token.strip()
        if token and not token.startswith(b"#"):
            return token


def read_ppm(path: Path) -> tuple[int, int, bytes]:
    with path.open("rb") as stream:
        if read_token(stream) != b"P6":
            raise ValueError(f"{path} is not a binary PPM image")
        width, height = (int(value) for value in read_token(stream).split())
        maximum = int(read_token(stream))
        if maximum != 255:
            raise ValueError(f"{path} uses unsupported channel maximum {maximum}")
        pixels = stream.read()
    expected = width * height * 3
    if len(pixels) != expected:
        raise ValueError(f"{path} contains {len(pixels)} pixel bytes; expected {expected}")
    return width, height, pixels


def page_number(path: Path) -> int:
    return int(path.stem.rsplit("-", 1)[1])


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("reference", type=Path)
    parser.add_argument("actual", type=Path)
    parser.add_argument("--channel-tolerance", type=int, default=8)
    parser.add_argument("--pixel-ratio", type=float, default=0.0005)
    args = parser.parse_args()

    reference_pages = sorted(args.reference.glob("page-*.ppm"), key=page_number)
    actual_pages = sorted(args.actual.glob("page-*.ppm"), key=page_number)
    if len(reference_pages) != len(actual_pages):
        raise SystemExit(
            f"rendering page count changed: {len(reference_pages)} reference, "
            f"{len(actual_pages)} actual"
        )

    failed = False
    for reference_path, actual_path in zip(reference_pages, actual_pages):
        ref_width, ref_height, reference = read_ppm(reference_path)
        new_width, new_height, actual = read_ppm(actual_path)
        if (ref_width, ref_height) != (new_width, new_height):
            print(
                f"{actual_path.name}: dimensions changed from "
                f"{ref_width}x{ref_height} to {new_width}x{new_height}"
            )
            failed = True
            continue

        if reference == actual:
            continue

        changed = 0
        largest_delta = 0
        for offset in range(0, len(reference), 3):
            delta = max(
                abs(reference[offset + channel] - actual[offset + channel])
                for channel in range(3)
            )
            largest_delta = max(largest_delta, delta)
            if delta > args.channel_tolerance:
                changed += 1

        pixels = ref_width * ref_height
        ratio = changed / pixels
        if ratio > args.pixel_ratio:
            print(
                f"{actual_path.name}: {changed} pixels changed ({ratio:.4%}); "
                f"maximum channel delta {largest_delta}"
            )
            failed = True

    if failed:
        raise SystemExit("visual regression detected; inspect and regenerate tests/test.pdf")
    print(f"Visual regression comparison passed for {len(reference_pages)} pages.")


if __name__ == "__main__":
    main()
