#!/usr/bin/env python3
"""Generates minimal placeholder icon.png/icon.ico for local T04-01 builds
(no design asset exists yet; a real icon is out of this task's scope).
Pure stdlib (zlib + struct), no image library dependency admitted."""
import struct
import zlib
from pathlib import Path

HERE = Path(__file__).parent
SIZE = 256
RGBA = (90, 90, 200, 255)  # flat placeholder color


def make_png(size: int, rgba: tuple[int, int, int, int]) -> bytes:
    def chunk(tag: bytes, data: bytes) -> bytes:
        return struct.pack(">I", len(data)) + tag + data + struct.pack(">I", zlib.crc32(tag + data))

    sig = b"\x89PNG\r\n\x1a\n"
    ihdr = struct.pack(">IIBBBBB", size, size, 8, 6, 0, 0, 0)
    row = bytes([0]) + bytes(rgba) * size
    raw = row * size
    idat = zlib.compress(raw, 9)
    return sig + chunk(b"IHDR", ihdr) + chunk(b"IDAT", idat) + chunk(b"IEND", b"")


def make_ico(png_bytes: bytes, size: int) -> bytes:
    header = struct.pack("<HHH", 0, 1, 1)
    width = 0 if size >= 256 else size
    height = 0 if size >= 256 else size
    entry = struct.pack("<BBBBHHII", width, height, 0, 0, 1, 32, len(png_bytes), 22)
    return header + entry + png_bytes


def main() -> int:
    png = make_png(SIZE, RGBA)
    (HERE / "icon.png").write_bytes(png)
    (HERE / "icon.ico").write_bytes(make_ico(png, SIZE))
    print(f"wrote icon.png ({len(png)} bytes), icon.ico")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
