#!/usr/bin/env python3
"""T05-05: Windows PE reproducibility isolation check (stdlib only).

Two clean MSVC-linked release builds never match bit-for-bit: link.exe
writes the current time into the COFF header, generates a fresh PDB GUID
per link, and derives the PE checksum from the resulting bytes. None of
those fields is code or content. This script isolates exactly that
irreducible toolchain difference, per the canonical plan's own clause
("unsigned payloads match bit-for-bit or each irreducible toolchain
difference is isolated, documented and independently shown not to affect
code/content"):

  1. If the two files are byte-identical, report REPRODUCIBLE.
  2. Otherwise, mask the known linker-generated fields (COFF TimeDateStamp,
     Optional-header CheckSum, Debug-directory entry TimeDateStamps, and
     CodeView PDB GUID + Age) in both copies and compare the remainder.
  3. Report ISOLATED_TOOLCHAIN_DIFFERENCE only when the masked remainder is
     byte-identical AND every actually-differing offset lies inside a masked
     range. Any other divergence reports DIVERGENT.

Usage:
  python3 pe_reproducibility.py <build1.exe> <build2.exe>

Exit 0 on REPRODUCIBLE or ISOLATED_TOOLCHAIN_DIFFERENCE, 1 on DIVERGENT or
on any unparsable input. All output is plain text, no secrets.
"""
import datetime
import struct
import sys
from pathlib import Path

MASK_NAMES = ("COFF_TimeDateStamp", "Optional_CheckSum", "Debug_TimeDateStamp", "CodeView_GUID_Age")


def parse_pe(data: bytes):
    """Return (masked_ranges, field_values) for a PE32+ image.

    masked_ranges: list of (offset, length, name). Raises ValueError on
    anything that is not a well-formed PE32+ image.
    """
    if len(data) < 0x40 or data[0:2] != b"MZ":
        raise ValueError("not a DOS MZ image")
    (e_lfanew,) = struct.unpack_from("<I", data, 0x3C)
    if data[e_lfanew:e_lfanew + 4] != b"PE\0\0":
        raise ValueError("PE signature missing")
    coff = e_lfanew + 4
    # COFF file header: Machine(+0,2) NumberOfSections(+2,2)
    # TimeDateStamp(+4,4) PointerToSymbolTable(+8,4) NumberOfSymbols(+12,4)
    # SizeOfOptionalHeader(+16,2) Characteristics(+18,2).
    (num_sections,) = struct.unpack_from("<H", data, coff + 2)
    (opt_size,) = struct.unpack_from("<H", data, coff + 16)
    opt = coff + 20
    (magic,) = struct.unpack_from("<H", data, opt)
    if magic != 0x20B:
        raise ValueError(f"not PE32+ (magic {magic:#x})")

    ranges = []
    values = {}

    ts_off = coff + 4
    (ts,) = struct.unpack_from("<I", data, ts_off)
    ranges.append((ts_off, 4, MASK_NAMES[0]))
    values["coff_timestamp"] = datetime.datetime.fromtimestamp(ts, datetime.timezone.utc).isoformat()

    csum_off = opt + 64
    (csum,) = struct.unpack_from("<I", data, csum_off)
    ranges.append((csum_off, 4, MASK_NAMES[1]))
    values["checksum"] = f"{csum:#010x}"

    dd_off = opt + 112 + 6 * 8
    (debug_rva, debug_size) = struct.unpack_from("<II", data, dd_off)
    values["debug_entries"] = debug_size // 28 if debug_size else 0
    if debug_size:
        sections = []
        sh_off = opt + opt_size
        for _ in range(num_sections):
            (vaddr, raw_size, raw_ptr) = struct.unpack_from("<III", data, sh_off + 12)
            sections.append((vaddr, raw_size, raw_ptr))
            sh_off += 40

        def rva_to_offset(rva):
            for vaddr, raw_size, raw_ptr in sections:
                if vaddr <= rva < vaddr + raw_size:
                    return raw_ptr + (rva - vaddr)
            raise ValueError(f"debug RVA {rva:#x} maps to no section")

        dbg_off = rva_to_offset(debug_rva)
        for i in range(debug_size // 28):
            entry = dbg_off + i * 28
            if entry + 28 > len(data):
                raise ValueError("debug directory entry runs past end of file")
            # IMAGE_DEBUG_DIRECTORY is 28 bytes. Only well-established
            # offsets are used: TimeDateStamp (+4, DWORD), Type (+12,
            # WORD), PointerToRawData (+24, DWORD, last field).
            (entry_ts,) = struct.unpack_from("<I", data, entry + 4)
            ranges.append((entry + 4, 4, MASK_NAMES[2]))
            (dtype,) = struct.unpack_from("<H", data, entry + 12)
            if dtype == 2:  # CodeView: RSDS + GUID(16) + Age(4) + PDB path
                (raw_ptr,) = struct.unpack_from("<I", data, entry + 24)
                if data[raw_ptr:raw_ptr + 4] != b"RSDS":
                    raise ValueError("CodeView entry without RSDS signature")
                ranges.append((raw_ptr + 4, 20, MASK_NAMES[3]))
                guid = data[raw_ptr + 4:raw_ptr + 20]
                (age,) = struct.unpack_from("<I", data, raw_ptr + 20)
                values.setdefault("codeview", []).append(f"guid={guid.hex()} age={age}")
    return ranges, values


def masked_copy(data: bytes, ranges):
    masked = bytearray(data)
    for off, length, _ in ranges:
        for i in range(off, off + length):
            masked[i] = 0
    return bytes(masked)


def main() -> int:
    if len(sys.argv) != 3:
        print(f"usage: {Path(sys.argv[0]).name} <build1.exe> <build2.exe>", file=sys.stderr)
        return 2
    a = Path(sys.argv[1]).read_bytes()
    b = Path(sys.argv[2]).read_bytes()
    if len(a) != len(b):
        print(f"DIVERGENT file sizes differ: {len(a)} vs {len(b)}")
        return 1
    if a == b:
        print("REPRODUCIBLE byte-identical")
        return 0
    try:
        ranges_a, values_a = parse_pe(a)
        ranges_b, values_b = parse_pe(b)
    except (ValueError, struct.error) as exc:
        print(f"DIVERGENT unparsable PE image: {exc}")
        return 1
    key_a = sorted((off, length, name) for off, length, name in ranges_a)
    key_b = sorted((off, length, name) for off, length, name in ranges_b)
    if key_a != key_b:
        print("DIVERGENT masked field layout differs between builds")
        print(f"  build1: {key_a}")
        print(f"  build2: {key_b}")
        return 1
    if masked_copy(a, ranges_a) != masked_copy(b, ranges_b):
        diffs = [i for i, (x, y) in enumerate(zip(a, b)) if x != y]
        masked_offsets = set()
        for off, length, _ in ranges_a:
            masked_offsets.update(range(off, off + length))
        unexplained = [i for i in diffs if i not in masked_offsets][:20]
        print(f"DIVERGENT {len(diffs)} differing bytes, {len(unexplained)} outside linker-generated fields")
        print(f"  first unexplained offsets: {unexplained}")
        return 1
    print("ISOLATED_TOOLCHAIN_DIFFERENCE only linker-generated fields differ (timestamps, PE checksum, PDB identity)")
    for off, length, name in key_a:
        print(f"  masked {name} @+{off} len={length}")
    print(f"  build1 fields: {values_a}")
    print(f"  build2 fields: {values_b}")
    print("  remainder: BIT_IDENTICAL (code/content unaffected)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
