#!/usr/bin/env python3
"""Compare two sets of binary PPM pages with a small raster tolerance."""

from __future__ import annotations

import argparse
from pathlib import Path


def read_ppm_header_token(stream) -> bytes:
    while True:
        token = stream.readline()
        if not token:
            raise ValueError("unexpected end of PPM header")
        token = token.strip()
        if token and not token.startswith(b"#"):
            return token


def read_ppm(path: Path) -> tuple[int, int, bytes]:
    with path.open("rb") as stream:
        if read_ppm_header_token(stream) != b"P6":
            raise ValueError(f"{path} is not a binary PPM image")
        width, height = (
            int(dimension) for dimension in read_ppm_header_token(stream).split()
        )
        channel_maximum = int(read_ppm_header_token(stream))
        if channel_maximum != 255:
            raise ValueError(
                f"{path} uses unsupported channel maximum {channel_maximum}"
            )
        pixels = stream.read()
    expected_pixel_bytes = width * height * 3
    if len(pixels) != expected_pixel_bytes:
        raise ValueError(
            f"{path} contains {len(pixels)} pixel bytes; "
            f"expected {expected_pixel_bytes}"
        )
    return width, height, pixels


def page_number(path: Path) -> int:
    return int(path.stem.rsplit("-", 1)[1])


def main() -> None:
    argument_parser = argparse.ArgumentParser()
    argument_parser.add_argument("reference", type=Path)
    argument_parser.add_argument("actual", type=Path)
    argument_parser.add_argument("--channel-tolerance", type=int, default=8)
    argument_parser.add_argument("--pixel-ratio", type=float, default=0.0005)
    arguments = argument_parser.parse_args()

    reference_pages = sorted(arguments.reference.glob("page-*.ppm"), key=page_number)
    actual_pages = sorted(arguments.actual.glob("page-*.ppm"), key=page_number)
    if len(reference_pages) != len(actual_pages):
        raise SystemExit(
            f"rendering page count changed: {len(reference_pages)} reference, "
            f"{len(actual_pages)} actual"
        )

    has_visual_regression = False
    for reference_path, actual_path in zip(reference_pages, actual_pages):
        ref_width, ref_height, reference = read_ppm(reference_path)
        new_width, new_height, actual = read_ppm(actual_path)
        if (ref_width, ref_height) != (new_width, new_height):
            print(
                f"{actual_path.name}: dimensions changed from "
                f"{ref_width}x{ref_height} to {new_width}x{new_height}"
            )
            has_visual_regression = True
            continue

        if reference == actual:
            continue

        changed_pixels = 0
        largest_channel_delta = 0
        for offset in range(0, len(reference), 3):
            channel_delta = max(
                abs(reference[offset + channel] - actual[offset + channel])
                for channel in range(3)
            )
            largest_channel_delta = max(largest_channel_delta, channel_delta)
            if channel_delta > arguments.channel_tolerance:
                changed_pixels += 1

        total_pixels = ref_width * ref_height
        changed_ratio = changed_pixels / total_pixels
        if changed_ratio > arguments.pixel_ratio:
            print(
                f"{actual_path.name}: {changed_pixels} pixels changed "
                f"({changed_ratio:.4%}); maximum channel delta "
                f"{largest_channel_delta}"
            )
            has_visual_regression = True

    if has_visual_regression:
        raise SystemExit("visual regression detected; inspect and regenerate tests/test.pdf")
    print(f"Visual regression comparison passed for {len(reference_pages)} pages.")


if __name__ == "__main__":
    main()
