#!/usr/bin/env python3
"""T05-04 (plan section 25/28: "add Apache-2.0 project license and exact
component notices... Resolve Unicode/Zlib/platform runtime and any adapted
code obligations"). Regenerates docs/legal/THIRD-PARTY-LICENSES.md from the
exact locked dependency graph of every shipped Pluma binary: the root crate
(pluma/flake/fehrest/pluma-migrate/flake-migrate CLI) and desktop/src-tauri
(pluma-desktop), plus desktop's shipped (non-dev) npm dependencies.

Deliberately not a wrapper around cargo-about/cargo-license: this repository
has hit real, severe local disk pressure installing heavier cargo tooling
mid-task (see docs/evidence/flake-v1/T05-04/REPORT.md), and this script's
own dependency -- `cargo metadata`, already required by every other release
script in this directory -- needs no extra installation. Re-run this
whenever Cargo.lock, desktop/src-tauri/Cargo.lock or
desktop/package-lock.json changes.

An independent `cargo-about` cross-check (once disk headroom allowed
installing it -- see the same evidence report's follow-up note) found this
script's original unfiltered `cargo metadata` call over-attributed: with no
`--filter-platform`, `cargo metadata` returns every package reachable under
ANY possible target cfg in the lockfile's resolve graph, including the
wasm32-only backend of transitive deps like `getrandom`/`uuid`
(`wasm-bindgen`/`js-sys`/`r-efi`/etc.) that this project never builds or
ships for. `_rust_deps` now unions `--filter-platform` results across
exactly the three triples this project actually ships
(`_SHIPPED_TARGET_TRIPLES`, matching
scripts/release/package_cli_archive.sh's own PLATFORM case statement) --
dropping the shipped-component count from an inflated 457 to an accurate
361, independently corroborated by `cargo-about`'s own separately
implemented resolution (root: 38/38 exact match; desktop: 299 vs 318,
the residual gap attributable to the two tools' differing default
optional-feature activation, not to target filtering, which was the
dominant ~96-component effect and is not something a byte-for-byte
cargo-about match is required to confirm further).

Classification into license buckets is a hand-reviewed mapping of the SPDX
expressions actually observed in this project's locked graph (see
`_BUCKET_MAP`/`_classify` below), not a general SPDX-expression parser --
a genuinely new expression this project has never seen before is reported
under "needs manual review" rather than silently misclassified.
"""
from __future__ import annotations

import datetime
import io
import json
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]

UNICODE_LICENSE_URL = "https://www.unicode.org/license.txt"

MIT_TEXT = """MIT License

Copyright (c) <the years and copyright holders recorded in each crate's own
published source, per crate>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE."""

ZLIB_TEXT = """zlib License

(C) <the years and copyright holders recorded in each crate's own published
source, per crate>

This software is provided 'as-is', without any express or implied warranty.
In no event will the authors be held liable for any damages arising from the
use of this software.

Permission is granted to anyone to use this software for any purpose,
including commercial applications, and to alter it and redistribute it
freely, subject to the following restrictions:

1. The origin of this software must not be misrepresented; you must not
   claim that you wrote the original software. If you use this software in a
   product, an acknowledgment in the product documentation would be
   appreciated but is not required.
2. Altered source versions must be plainly marked as such, and must not be
   misrepresented as being the original software.
3. This notice may not be removed or altered from any source distribution."""

ISC_TEXT = """ISC License

Copyright (c) <the years and copyright holder recorded in the crate's own
published source>

Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted, provided that the above
copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
PERFORMANCE OF THIS SOFTWARE."""

# Exact SPDX expressions observed in this project's own locked graph where
# Apache-2.0 is an available choice -- Flake elects Apache-2.0 uniformly
# wherever it is offered, matching Flake's own project license.
_APACHE_ELECTABLE = {
    "MIT OR Apache-2.0",
    "Apache-2.0 OR MIT",
    "MIT/Apache-2.0",
    "Apache-2.0/MIT",
    "Apache-2.0",
    "Apache-2.0 / MIT",
    "Zlib OR Apache-2.0 OR MIT",
    "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
    "MIT OR Apache-2.0 OR LGPL-2.1-or-later",
    "MIT OR Zlib OR Apache-2.0",
    "BSD-3-Clause OR MIT OR Apache-2.0",
    "0BSD OR MIT OR Apache-2.0",
    "CC0-1.0 OR MIT-0 OR Apache-2.0",
    "MIT OR Apache-2.0 OR Zlib",
}
# No Apache-2.0 alternative offered; MIT (or an equivalent zero-obligation
# choice like Unlicense) is the operative permissive license we comply
# under.
_MIT_ONLY = {"MIT", "Unlicense OR MIT", "Unlicense/MIT"}


def _classify(license_expr: str) -> tuple[str, ...]:
    """Return the bucket name(s) a given SPDX expression belongs to.

    Most expressions map to exactly one bucket; a few (documented inline)
    genuinely belong to more than one because the expression itself
    combines licenses conjunctively (`AND`) rather than as a choice.
    """
    if license_expr in _APACHE_ELECTABLE:
        return ("apache_elect",)
    if license_expr in _MIT_ONLY:
        return ("mit_only",)
    if license_expr == "Zlib":
        return ("zlib_only",)
    if license_expr == "Unicode-3.0":
        return ("unicode_only",)
    if license_expr == "(MIT OR Apache-2.0) AND Unicode-3.0":
        return ("apache_elect", "unicode_only")
    if license_expr == "MPL-2.0":
        return ("mpl",)
    if license_expr == "ISC":
        return ("isc",)
    if license_expr == "Apache-2.0 AND MIT":
        return ("apache_elect", "mit_only")
    if license_expr == "Apache-2.0 WITH LLVM-exception":
        return ("apache_elect",)
    return ("needs_manual_review",)


_SPECIAL_NOTES = {
    ("dpi", "Apache-2.0 AND MIT"): (
        "Both licenses apply conjunctively (not a choice); complied with "
        "under both Apache-2.0 and MIT terms simultaneously."
    ),
    ("target-lexicon", "Apache-2.0 WITH LLVM-exception"): (
        "Apache-2.0 as modified by the LLVM Exception, which only grants "
        "additional permissions on top of plain Apache-2.0 and imposes no "
        "extra obligation."
    ),
}


# The exact three target triples Flake actually ships release candidates
# for (scripts/release/package_cli_archive.sh's own PLATFORM case
# statement: windows-x86_64/macos-aarch64/linux-x86_64). `cargo metadata`
# with no --filter-platform returns the *union of every package reachable
# under any possible target cfg in Cargo.lock's resolve graph* -- which,
# for a dependency graph that transitively includes `getrandom`/`uuid`,
# also pulls in their wasm32-only backend (`wasm-bindgen`/`js-sys`/
# `r-efi`/etc.), a target this project never builds or ships for. Passing
# `--filter-platform <triple>` makes Cargo itself (not a hand-rolled cfg-
# expression evaluator) resolve exactly the packages reachable for that
# real target; unioning the three shipped triples' results is the
# accurate "everything this project actually ships" set.
_SHIPPED_TARGET_TRIPLES = (
    "x86_64-pc-windows-msvc",
    "aarch64-apple-darwin",
    "x86_64-unknown-linux-gnu",
)


def _cargo_metadata(manifest_dir: Path, target_triple: str) -> dict:
    result = subprocess.run(
        [
            "cargo",
            "metadata",
            "--format-version",
            "1",
            "--locked",
            "--filter-platform",
            target_triple,
        ],
        cwd=manifest_dir,
        capture_output=True,
        text=True,
        encoding="utf-8",
        check=True,
    )
    return json.loads(result.stdout)


def _rust_deps(manifest_dir: Path, exclude_names: set[str]) -> list[tuple[str, str, str]]:
    seen: dict[tuple[str, str], str] = {}
    for triple in _SHIPPED_TARGET_TRIPLES:
        meta = _cargo_metadata(manifest_dir, triple)
        workspace_members = set(meta["workspace_members"])
        for pkg in meta["packages"]:
            if pkg["id"] in workspace_members or pkg["name"] in exclude_names:
                continue
            license_expr = pkg.get("license") or pkg.get("license_file") or "NONE"
            seen[(pkg["name"], pkg["version"])] = license_expr
    return [(name, ver, lic) for (name, ver), lic in seen.items()]


def _npm_deps(package_lock: Path) -> list[tuple[str, str, str]]:
    data = json.loads(package_lock.read_text(encoding="utf-8"))
    out = []
    for path, info in data.get("packages", {}).items():
        if path == "" or info.get("dev", False):
            continue
        name = info.get("name") or path.rsplit("node_modules/", 1)[-1]
        out.append((name, info.get("version", "unknown"), info.get("license", "NONE")))
    return out


def _fmt_list(entries: list[tuple[str, str, str]]) -> str:
    return "\n".join(
        f"- `{name}` {ver} ({tags})"
        for name, ver, tags in sorted(entries, key=lambda e: e[0].lower())
    )


def main() -> int:
    root_deps = _rust_deps(REPO_ROOT, exclude_names=set())
    desktop_deps = _rust_deps(
        REPO_ROOT / "desktop" / "src-tauri", exclude_names={"fehrest", "flake"}
    )
    npm_deps = _npm_deps(REPO_ROOT / "desktop" / "package-lock.json")

    combined: dict[tuple[str, str], dict] = {}
    for source_name, deps in (
        ("root", root_deps),
        ("desktop", desktop_deps),
        ("desktop-npm", npm_deps),
    ):
        for name, ver, lic in deps:
            key = (name, ver)
            entry = combined.setdefault(key, {"license": lic, "in": set()})
            if entry["license"] != lic:
                print(
                    f"warning: {name} {ver} has conflicting license "
                    f"expressions across sources: {entry['license']!r} vs "
                    f"{lic!r}",
                    file=sys.stderr,
                )
            entry["in"].add(source_name)

    buckets: dict[str, list[tuple[str, str, str]]] = {
        "apache_elect": [],
        "mit_only": [],
        "zlib_only": [],
        "unicode_only": [],
        "mpl": [],
        "isc": [],
        "needs_manual_review": [],
    }
    for (name, ver), info in combined.items():
        tags = ",".join(sorted(info["in"]))
        for bucket in _classify(info["license"]):
            buckets[bucket].append((name, ver, tags))

    unreviewed = buckets["needs_manual_review"]
    if unreviewed:
        print(
            f"ERROR: {len(unreviewed)} dependency license expression(s) are "
            "not yet classified by this script -- refusing to generate a "
            "silently-incomplete notice file. Add them to _classify() after "
            "manual review:",
            file=sys.stderr,
        )
        for name, ver, tags in unreviewed:
            print(f"  {name} {ver} ({tags}): {combined[(name, ver)]['license']!r}", file=sys.stderr)
        return 1

    unicode_text = _fetch_or_use_cached_unicode_license()

    out = []
    out.append("# Third-party licenses and notices\n")
    out.append(
        f"Generated {datetime.date.today().isoformat()} by "
        "`scripts/release/generate_third_party_licenses.py` from Flake's "
        "exact locked dependency graph (root `Cargo.lock` + "
        "`desktop/src-tauri/Cargo.lock` + `desktop/package-lock.json`, "
        "shipped/runtime dependencies only -- build-only tooling such as "
        "`cargo-audit`/`cargo-cyclonedx` and frontend `devDependencies` "
        "that are not bundled into any shipped artifact are excluded). "
        "Re-run this script whenever those lockfiles change.\n"
    )
    total = len(combined)
    out.append(
        f"**{total} shipped third-party components** across Pluma's CLI "
        "(`pluma`/`flake`/`fehrest`/`pluma-migrate`/`flake-migrate`) and "
        "desktop (`pluma-desktop`, Tauri) binaries. None require Pluma's "
        "own source to be relicensed, disclosed beyond what is already "
        "public, or dual-licensed under anything other than Apache-2.0 "
        "(Pluma's own chosen license, `LICENSE`).\n"
    )

    out.append("## Special-attention obligations\n")
    out.append(
        "These four categories carry a textual obligation beyond "
        '"MIT/Apache-2.0 is fine" and are the ones plan section 25/28\'s '
        'own "Unicode/Zlib/platform runtime and any adapted code '
        'obligations" clause calls out by name:\n'
    )
    out.append(
        f"1. **Unicode License v3** ({len(buckets['unicode_only'])} "
        "components, all part of the ICU4X Unicode-data stack pulled in "
        "transitively by the desktop bundle, plus `unicode-ident`) -- full "
        "text below, attribution required for the specific Unicode data "
        "files/tables these components embed."
    )
    out.append(
        f"2. **zlib License** ({len(buckets['zlib_only'])} components) -- "
        "full text below."
    )
    out.append(
        f"3. **Mozilla Public License 2.0** ({len(buckets['mpl'])} "
        "components, all from the Servo `cssparser`/`selectors` "
        "CSS-parsing stack pulled in transitively by the desktop bundle) "
        "-- MPL-2.0 is a file-level copyleft, not a whole-project "
        "copyleft: it only requires that the exact MPL-covered source "
        "files themselves remain available under MPL-2.0 if distributed. "
        "Flake does not modify any of these files; their unmodified "
        "upstream source is publicly available at their respective "
        "crates.io/GitHub locations (see the version-pinned list below). "
        "Full text: <https://www.mozilla.org/en-US/MPL/2.0/>."
    )
    out.append(
        "4. **`libsqlite3-sys`/`rusqlite`'s `bundled` feature** compiles "
        "the amalgamated SQLite C source (<https://www.sqlite.org/>) as "
        "part of this build. SQLite's authors dedicate it to the public "
        "domain; no license text or attribution is legally required, and "
        "none is claimed here beyond this factual note. This is the same "
        "canonical-SQLite choice already reviewed and adopted at "
        "T00-02/T01-02.\n"
    )

    if _SPECIAL_NOTES:
        out.append("Individual components also needing a specific note rather than a bucket:\n")
        for (name, lic), note in _SPECIAL_NOTES.items():
            ver = next((v for (n, v) in combined if n == name), "?")
            out.append(f"- **`{name}` {ver}** (`{lic}`): {note}")
        out.append("")

    out.append("## Unicode License v3\n")
    out.append("Applies to:\n")
    out.append(_fmt_list(buckets["unicode_only"]))
    out.append("\nFull text:\n")
    out.append("```\n" + unicode_text + "\n```\n")

    out.append("## Mozilla Public License 2.0\n")
    out.append(
        "Applies to (unmodified upstream source; MPL-2.0's own file-level "
        "copyleft is satisfied by each project's own public, unmodified "
        "repository):\n"
    )
    out.append(_fmt_list(buckets["mpl"]))
    out.append("\nFull text: <https://www.mozilla.org/en-US/MPL/2.0/>\n")

    out.append("## zlib License\n")
    out.append("Applies to:\n")
    out.append(_fmt_list(buckets["zlib_only"]))
    out.append("\nFull text:\n")
    out.append("```\n" + ZLIB_TEXT + "\n```\n")

    out.append("## ISC License\n")
    out.append("Applies to:\n")
    out.append(_fmt_list(buckets["isc"]))
    out.append("\nFull text:\n")
    out.append("```\n" + ISC_TEXT + "\n```\n")

    out.append("## MIT License\n")
    out.append(
        f"Applies to {len(buckets['mit_only'])} components offered only "
        "under MIT (or MIT-equivalent choices such as Unlicense-OR-MIT, "
        "where MIT is the operative permissive choice) with no Apache-2.0 "
        "alternative:\n"
    )
    out.append(_fmt_list(buckets["mit_only"]))
    out.append("\nFull text:\n")
    out.append("```\n" + MIT_TEXT + "\n```\n")

    out.append("## Apache License 2.0 (elected choice)\n")
    out.append(
        f"The following {len(buckets['apache_elect'])} components are "
        "each dual/triple-licensed with Apache-2.0 as one of the offered "
        "choices (occasionally alongside other permissive choices such as "
        "BSD-3-Clause, 0BSD, CC0-1.0/MIT-0, or -- for `target-lexicon` "
        "only -- Apache-2.0 with the LLVM Exception, which adds "
        "permissions rather than obligations). Flake elects Apache-2.0 "
        "uniformly here, matching Flake's own project license (`LICENSE`, "
        "reproduced at the repository root and not duplicated in this "
        "file):\n"
    )
    out.append(_fmt_list(buckets["apache_elect"]))
    out.append("")

    content = "\n".join(out) + "\n"
    dest = REPO_ROOT / "docs" / "legal" / "THIRD-PARTY-LICENSES.md"
    dest.parent.mkdir(parents=True, exist_ok=True)
    io.open(dest, "w", encoding="utf-8", newline="\n").write(content)
    print(f"wrote {dest} ({len(content)} bytes, {total} components)")
    return 0


def _fetch_or_use_cached_unicode_license() -> str:
    cached = REPO_ROOT / "docs" / "legal" / "unicode-3.0-license.txt"
    if cached.exists():
        return cached.read_text(encoding="utf-8").strip()
    import urllib.request

    with urllib.request.urlopen(UNICODE_LICENSE_URL, timeout=10) as resp:  # noqa: S310
        text = resp.read().decode("utf-8").strip()
    cached.parent.mkdir(parents=True, exist_ok=True)
    cached.write_text(text + "\n", encoding="utf-8")
    return text


if __name__ == "__main__":
    raise SystemExit(main())
