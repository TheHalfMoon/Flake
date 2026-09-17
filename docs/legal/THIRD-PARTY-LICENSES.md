# Third-party licenses and notices

Generated 2026-09-17 by `scripts/release/generate_third_party_licenses.py` from Flake's exact locked dependency graph (root `Cargo.lock` + `desktop/src-tauri/Cargo.lock` + `desktop/package-lock.json`, shipped/runtime dependencies only -- build-only tooling such as `cargo-audit`/`cargo-cyclonedx` and frontend `devDependencies` that are not bundled into any shipped artifact are excluded). Re-run this script whenever those lockfiles change.

**457 shipped third-party components** across Flake's CLI (`flake`/`fehrest`/`flake-migrate`) and desktop (`flake-desktop`, Tauri) binaries. None require Flake's own source to be relicensed, disclosed beyond what is already public, or dual-licensed under anything other than Apache-2.0 (Flake's own chosen license, `LICENSE`).

## Special-attention obligations

These four categories carry a textual obligation beyond "MIT/Apache-2.0 is fine" and are the ones plan section 25/28's own "Unicode/Zlib/platform runtime and any adapted code obligations" clause calls out by name:

1. **Unicode License v3** (19 components, all part of the ICU4X Unicode-data stack pulled in transitively by the desktop bundle, plus `unicode-ident`) -- full text below, attribution required for the specific Unicode data files/tables these components embed.
2. **zlib License** (3 components) -- full text below.
3. **Mozilla Public License 2.0** (5 components, all from the Servo `cssparser`/`selectors` CSS-parsing stack pulled in transitively by the desktop bundle) -- MPL-2.0 is a file-level copyleft, not a whole-project copyleft: it only requires that the exact MPL-covered source files themselves remain available under MPL-2.0 if distributed. Flake does not modify any of these files; their unmodified upstream source is publicly available at their respective crates.io/GitHub locations (see the version-pinned list below). Full text: <https://www.mozilla.org/en-US/MPL/2.0/>.
4. **`libsqlite3-sys`/`rusqlite`'s `bundled` feature** compiles the amalgamated SQLite C source (<https://www.sqlite.org/>) as part of this build. SQLite's authors dedicate it to the public domain; no license text or attribution is legally required, and none is claimed here beyond this factual note. This is the same canonical-SQLite choice already reviewed and adopted at T00-02/T01-02.

Individual components also needing a specific note rather than a bucket:

- **`dpi` 0.1.2** (`Apache-2.0 AND MIT`): Both licenses apply conjunctively (not a choice); complied with under both Apache-2.0 and MIT terms simultaneously.
- **`target-lexicon` 0.12.16** (`Apache-2.0 WITH LLVM-exception`): Apache-2.0 as modified by the LLVM Exception, which only grants additional permissions on top of plain Apache-2.0 and imposes no extra obligation.

## Unicode License v3

Applies to:

- `icu_collections` 2.3.0 (desktop)
- `icu_locale_core` 2.3.0 (desktop)
- `icu_normalizer` 2.3.0 (desktop)
- `icu_normalizer_data` 2.3.0 (desktop)
- `icu_properties` 2.3.0 (desktop)
- `icu_properties_data` 2.3.0 (desktop)
- `icu_provider` 2.3.1 (desktop)
- `litemap` 0.8.3 (desktop)
- `potential_utf` 0.1.6 (desktop)
- `tinystr` 0.8.4 (desktop)
- `unicode-ident` 1.0.24 (desktop,root)
- `writeable` 0.6.4 (desktop)
- `yoke` 0.8.3 (desktop)
- `yoke-derive` 0.8.3 (desktop)
- `zerofrom` 0.1.8 (desktop)
- `zerofrom-derive` 0.1.8 (desktop)
- `zerotrie` 0.2.5 (desktop)
- `zerovec` 0.11.8 (desktop)
- `zerovec-derive` 0.11.6 (desktop)

Full text:

```
UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 1991-2026 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
Data Files or Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, and/or sell
copies of the Data Files or Software, and to permit persons to whom the
Data Files or Software are furnished to do so, provided that either (a)
this copyright and permission notice appear with all copies of the Data
Files or Software, or (b) this copyright and permission notice appear in
associated Documentation.

THE DATA FILES AND SOFTWARE ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY
KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF
THIRD PARTY RIGHTS.

IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS INCLUDED IN THIS NOTICE
BE LIABLE FOR ANY CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES,
OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THE DATA
FILES OR SOFTWARE.

Except as contained in this notice, the name of a copyright holder shall
not be used in advertising or otherwise to promote the sale, use or other
dealings in these Data Files or Software without prior written
authorization of the copyright holder.
```

## Mozilla Public License 2.0

Applies to (unmodified upstream source; MPL-2.0's own file-level copyleft is satisfied by each project's own public, unmodified repository):

- `cssparser` 0.36.0 (desktop)
- `cssparser-macros` 0.6.1 (desktop)
- `dtoa-short` 0.3.5 (desktop)
- `option-ext` 0.2.0 (desktop)
- `selectors` 0.36.1 (desktop)

Full text: <https://www.mozilla.org/en-US/MPL/2.0/>

## zlib License

Applies to:

- `foldhash` 0.1.5 (desktop,root)
- `foldhash` 0.2.0 (desktop)
- `zlib-rs` 0.6.8 (desktop)

Full text:

```
zlib License

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
3. This notice may not be removed or altered from any source distribution.
```

## ISC License

Applies to:

- `libloading` 0.7.4 (desktop)

Full text:

```
ISC License

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
PERFORMANCE OF THIS SOFTWARE.
```

## MIT License

Applies to 113 components offered only under MIT (or MIT-equivalent choices such as Unlicense-OR-MIT, where MIT is the operative permissive choice) with no Apache-2.0 alternative:

- `aho-corasick` 1.1.5 (desktop)
- `atk` 0.18.2 (desktop)
- `atk-sys` 0.18.2 (desktop)
- `block2` 0.6.2 (desktop)
- `byteorder` 1.5.0 (desktop)
- `bytes` 1.12.1 (desktop)
- `cairo-rs` 0.18.5 (desktop)
- `cairo-sys-rs` 0.18.2 (desktop)
- `cargo_metadata` 0.19.2 (desktop)
- `cfb` 0.7.3 (desktop)
- `combine` 4.6.8 (desktop)
- `darling` 0.24.1 (desktop)
- `darling_core` 0.24.1 (desktop)
- `darling_macro` 0.24.1 (desktop)
- `derive_more` 2.1.1 (desktop)
- `derive_more-impl` 2.1.1 (desktop)
- `dlopen2` 0.8.2 (desktop)
- `dlopen2_derive` 0.4.3 (desktop)
- `dom_query` 0.27.0 (desktop)
- `dpi` 0.1.2 (desktop)
- `embed-resource` 3.0.11 (desktop)
- `gdk` 0.18.2 (desktop)
- `gdk-pixbuf` 0.18.5 (desktop)
- `gdk-pixbuf-sys` 0.18.0 (desktop)
- `gdk-sys` 0.18.2 (desktop)
- `gdkwayland-sys` 0.18.2 (desktop)
- `generic-array` 0.14.7 (desktop,root)
- `gio` 0.18.4 (desktop)
- `gio-sys` 0.18.1 (desktop)
- `glib` 0.18.5 (desktop)
- `glib-macros` 0.18.5 (desktop)
- `glib-sys` 0.18.1 (desktop)
- `gobject-sys` 0.18.0 (desktop)
- `gtk` 0.18.2 (desktop)
- `gtk-sys` 0.18.2 (desktop)
- `gtk3-macros` 0.18.2 (desktop)
- `http-body` 1.1.0 (desktop)
- `http-body-util` 0.1.5 (desktop)
- `hyper` 1.11.1 (desktop)
- `hyper-util` 0.1.20 (desktop)
- `ico` 0.5.0 (desktop)
- `infer` 0.19.0 (desktop)
- `javascriptcore-rs` 1.1.2 (desktop)
- `javascriptcore-rs-sys` 1.1.1 (desktop)
- `jiff` 0.2.37 (desktop)
- `jiff-core` 0.1.1 (desktop)
- `jiff-static` 0.2.37 (desktop)
- `jiff-tzdb` 0.1.8 (desktop)
- `jiff-tzdb-platform` 0.1.3 (desktop)
- `libredox` 0.1.24 (desktop)
- `libsqlite3-sys` 0.35.0 (desktop,root)
- `memchr` 2.8.3 (desktop,root)
- `memoffset` 0.9.1 (desktop)
- `mio` 1.2.3 (desktop)
- `new_debug_unreachable` 1.0.6 (desktop)
- `objc2` 0.6.4 (desktop)
- `objc2-encode` 4.1.0 (desktop)
- `objc2-foundation` 0.3.2 (desktop)
- `pango` 0.18.3 (desktop)
- `pango-sys` 0.18.0 (desktop)
- `phf` 0.13.1 (desktop)
- `phf_codegen` 0.13.1 (desktop)
- `phf_generator` 0.13.1 (desktop)
- `phf_macros` 0.13.1 (desktop)
- `phf_shared` 0.13.1 (desktop)
- `plist` 1.10.1 (desktop)
- `precomputed-hash` 0.1.1 (desktop)
- `quick-xml` 0.42.0 (desktop)
- `react` 19.3.0 (desktop-npm)
- `react-dom` 19.3.0 (desktop-npm)
- `redox_syscall` 0.5.18 (desktop)
- `redox_users` 0.5.2 (desktop)
- `rfd` 0.16.0 (desktop)
- `rusqlite` 0.37.0 (desktop,root)
- `same-file` 1.0.6 (desktop)
- `scheduler` 0.28.0 (desktop-npm)
- `schemars` 0.8.22 (desktop)
- `schemars` 0.9.0 (desktop)
- `schemars` 1.2.2 (desktop)
- `schemars_derive` 0.8.22 (desktop)
- `simd-adler32` 0.3.10 (desktop)
- `slab` 0.4.12 (desktop,root)
- `soup3` 0.5.0 (desktop)
- `soup3-sys` 0.5.0 (desktop)
- `strsim` 0.11.1 (desktop)
- `synstructure` 0.14.0 (desktop)
- `tauri-winres` 0.3.6 (desktop)
- `tokio` 1.53.1 (desktop)
- `tokio-util` 0.7.19 (desktop)
- `tower` 0.5.3 (desktop)
- `tower-http` 0.6.11 (desktop)
- `tower-layer` 0.3.3 (desktop)
- `tower-service` 0.3.3 (desktop)
- `tracing` 0.1.44 (desktop)
- `tracing-core` 0.1.36 (desktop)
- `try-lock` 0.2.5 (desktop)
- `urlpattern` 0.3.0 (desktop)
- `version-compare` 0.2.1 (desktop)
- `vswhom` 0.1.0 (desktop)
- `vswhom-sys` 0.1.3 (desktop)
- `walkdir` 2.5.0 (desktop)
- `want` 0.3.1 (desktop)
- `webkit2gtk` 2.0.2 (desktop)
- `webkit2gtk-sys` 2.0.2 (desktop)
- `webview2-com` 0.38.2 (desktop)
- `webview2-com-macros` 0.8.1 (desktop)
- `webview2-com-sys` 0.38.2 (desktop)
- `winapi-util` 0.1.11 (desktop)
- `winnow` 0.5.40 (desktop)
- `winnow` 0.7.15 (desktop)
- `winnow` 1.0.4 (desktop)
- `winreg` 0.55.0 (desktop)
- `zmij` 1.0.23 (desktop,root)

Full text:

```
MIT License

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
SOFTWARE.
```

## Apache License 2.0 (elected choice)

The following 318 components are each dual/triple-licensed with Apache-2.0 as one of the offered choices (occasionally alongside other permissive choices such as BSD-3-Clause, 0BSD, CC0-1.0/MIT-0, or -- for `target-lexicon` only -- Apache-2.0 with the LLVM Exception, which adds permissions rather than obligations). Flake elects Apache-2.0 uniformly here, matching Flake's own project license (`LICENSE`, reproduced at the repository root and not duplicated in this file):

- `@tauri-apps/api` 2.11.1 (desktop-npm)
- `adler2` 2.0.1 (desktop)
- `android_system_properties` 0.1.6 (desktop)
- `anyhow` 1.0.104 (desktop)
- `atomic-waker` 1.1.2 (desktop)
- `autocfg` 1.5.1 (desktop)
- `base64` 0.21.7 (desktop)
- `base64` 0.22.1 (desktop)
- `base64` 0.23.1 (desktop)
- `bit-set` 0.8.0 (desktop)
- `bit-vec` 0.8.0 (desktop)
- `bitflags` 2.13.1 (root)
- `bitflags` 1.3.2 (desktop)
- `bitflags` 2.13.2 (desktop)
- `block-buffer` 0.10.4 (desktop,root)
- `bs58` 0.5.1 (desktop)
- `bumpalo` 3.20.3 (desktop,root)
- `bytemuck` 1.25.2 (desktop)
- `camino` 1.2.6 (desktop)
- `cargo-platform` 0.1.9 (desktop)
- `cargo_toml` 0.22.3 (desktop)
- `cc` 1.4.3 (root)
- `cc` 1.4.6 (desktop)
- `cesu8` 1.1.0 (desktop)
- `cfg-expr` 0.15.8 (desktop)
- `cfg-if` 1.0.4 (desktop,root)
- `chrono` 0.4.45 (desktop)
- `cookie` 0.18.2 (desktop)
- `core-foundation` 0.10.1 (desktop)
- `core-foundation-sys` 0.8.7 (desktop)
- `core-graphics` 0.25.0 (desktop)
- `core-graphics-types` 0.2.0 (desktop)
- `cpufeatures` 0.2.17 (desktop,root)
- `crc32fast` 1.5.2 (desktop)
- `crossbeam-channel` 0.5.17 (desktop)
- `crossbeam-utils` 0.8.23 (desktop)
- `crypto-common` 0.1.7 (desktop,root)
- `ctor` 0.8.0 (desktop)
- `ctor-proc-macro` 0.0.7 (desktop)
- `defmt` 1.1.1 (desktop)
- `defmt-macros` 1.1.1 (desktop)
- `defmt-parser` 1.0.0 (desktop)
- `deranged` 0.5.8 (desktop)
- `digest` 0.10.7 (desktop,root)
- `dirs` 6.0.0 (desktop)
- `dirs-sys` 0.5.0 (desktop)
- `dispatch2` 0.3.1 (desktop)
- `displaydoc` 0.2.7 (desktop)
- `dpi` 0.1.2 (desktop)
- `dtoa` 1.0.11 (desktop)
- `dtor` 0.3.0 (desktop)
- `dtor-proc-macro` 0.0.6 (desktop)
- `dunce` 1.0.5 (desktop)
- `dyn-clone` 1.0.20 (desktop)
- `embed_plist` 1.2.2 (desktop)
- `equivalent` 1.0.2 (desktop)
- `erased-serde` 0.4.10 (desktop)
- `fallible-iterator` 0.3.0 (desktop,root)
- `fallible-streaming-iterator` 0.1.9 (desktop,root)
- `fastrand` 2.5.0 (desktop)
- `fdeflate` 0.3.7 (desktop)
- `field-offset` 0.3.6 (desktop)
- `find-msvc-tools` 0.1.11 (root)
- `find-msvc-tools` 0.1.12 (desktop)
- `flate2` 1.1.10 (desktop)
- `fnv` 1.0.7 (desktop)
- `foreign-types` 0.5.0 (desktop)
- `foreign-types-macros` 0.2.4 (desktop)
- `foreign-types-shared` 0.3.1 (desktop)
- `form_urlencoded` 1.2.2 (desktop)
- `futures-channel` 0.3.34 (desktop)
- `futures-core` 0.3.34 (desktop,root)
- `futures-executor` 0.3.34 (desktop)
- `futures-io` 0.3.34 (desktop)
- `futures-macro` 0.3.34 (desktop)
- `futures-sink` 0.3.34 (desktop)
- `futures-task` 0.3.34 (desktop,root)
- `futures-util` 0.3.34 (desktop,root)
- `getrandom` 0.4.3 (desktop,root)
- `getrandom` 0.2.17 (desktop)
- `getrandom` 0.3.4 (desktop)
- `glob` 0.3.4 (desktop)
- `hashbrown` 0.15.5 (desktop,root)
- `hashbrown` 0.12.3 (desktop)
- `hashbrown` 0.17.1 (desktop)
- `hashlink` 0.10.0 (desktop,root)
- `heck` 0.4.1 (desktop)
- `heck` 0.5.0 (desktop)
- `hex` 0.4.3 (desktop)
- `html5ever` 0.38.0 (desktop)
- `http` 1.5.0 (desktop)
- `httparse` 1.10.1 (desktop)
- `iana-time-zone` 0.1.65 (desktop)
- `iana-time-zone-haiku` 0.1.2 (desktop)
- `ident_case` 1.0.1 (desktop)
- `idna` 1.1.0 (desktop)
- `idna_adapter` 1.2.2 (desktop)
- `indexmap` 1.9.3 (desktop)
- `indexmap` 2.14.2 (desktop)
- `ipnet` 2.12.2 (desktop)
- `itoa` 1.0.18 (desktop,root)
- `jni` 0.21.1 (desktop)
- `jni-sys` 0.3.1 (desktop)
- `jni-sys` 0.4.1 (desktop)
- `jni-sys-macros` 0.4.1 (desktop)
- `js-sys` 0.3.104 (root)
- `js-sys` 0.3.105 (desktop)
- `json-patch` 3.0.1 (desktop)
- `jsonptr` 0.6.3 (desktop)
- `keyboard-types` 0.7.0 (desktop)
- `libappindicator` 0.9.0 (desktop)
- `libappindicator-sys` 0.9.0 (desktop)
- `libc` 0.2.189 (desktop,root)
- `lock_api` 0.4.14 (desktop)
- `log` 0.4.34 (desktop)
- `markup5ever` 0.38.0 (desktop)
- `mime` 0.3.17 (desktop)
- `miniz_oxide` 0.8.9 (desktop)
- `miniz_oxide` 0.9.1 (desktop)
- `muda` 0.19.3 (desktop)
- `ndk` 0.9.0 (desktop)
- `ndk-sys` 0.6.0+11769913 (desktop)
- `num-conv` 0.2.2 (desktop)
- `num-traits` 0.2.19 (desktop)
- `num_enum` 0.7.6 (desktop)
- `num_enum_derive` 0.7.6 (desktop)
- `objc2-app-kit` 0.3.2 (desktop)
- `objc2-cloud-kit` 0.3.2 (desktop)
- `objc2-core-data` 0.3.2 (desktop)
- `objc2-core-foundation` 0.3.2 (desktop)
- `objc2-core-graphics` 0.3.2 (desktop)
- `objc2-core-image` 0.3.2 (desktop)
- `objc2-core-location` 0.3.2 (desktop)
- `objc2-core-text` 0.3.2 (desktop)
- `objc2-exception-helper` 0.1.1 (desktop)
- `objc2-io-surface` 0.3.2 (desktop)
- `objc2-quartz-core` 0.3.2 (desktop)
- `objc2-ui-kit` 0.3.2 (desktop)
- `objc2-user-notifications` 0.3.2 (desktop)
- `objc2-web-kit` 0.3.2 (desktop)
- `once_cell` 1.21.4 (desktop,root)
- `parking_lot` 0.12.5 (desktop)
- `parking_lot_core` 0.9.12 (desktop)
- `percent-encoding` 2.3.2 (desktop)
- `pin-project-lite` 0.2.17 (desktop,root)
- `pkg-config` 0.3.34 (desktop,root)
- `png` 0.17.16 (desktop)
- `png` 0.18.1 (desktop)
- `portable-atomic` 1.15.0 (desktop)
- `portable-atomic-util` 0.2.8 (desktop)
- `powerfmt` 0.2.0 (desktop)
- `proc-macro-crate` 1.3.1 (desktop)
- `proc-macro-crate` 2.0.2 (desktop)
- `proc-macro-crate` 3.5.0 (desktop)
- `proc-macro-error` 1.0.4 (desktop)
- `proc-macro-error-attr` 1.0.4 (desktop)
- `proc-macro2` 1.0.107 (desktop,root)
- `quote` 1.0.47 (desktop,root)
- `r-efi` 6.0.0 (desktop,root)
- `r-efi` 5.3.0 (desktop)
- `raw-window-handle` 0.6.2 (desktop)
- `ref-cast` 1.0.27 (desktop)
- `ref-cast-impl` 1.0.27 (desktop)
- `regex` 1.13.1 (desktop)
- `regex-automata` 0.4.18 (desktop)
- `regex-syntax` 0.8.11 (desktop)
- `reqwest` 0.13.5 (desktop)
- `rustc-hash` 2.1.3 (desktop)
- `rustc_version` 0.4.1 (desktop)
- `rustversion` 1.0.23 (desktop,root)
- `scopeguard` 1.2.0 (desktop)
- `semver` 1.0.28 (desktop)
- `serde` 1.0.229 (desktop,root)
- `serde-untagged` 0.1.9 (desktop)
- `serde_core` 1.0.229 (desktop,root)
- `serde_derive` 1.0.229 (desktop,root)
- `serde_derive_internals` 0.29.1 (desktop)
- `serde_json` 1.0.151 (desktop,root)
- `serde_repr` 0.1.21 (desktop)
- `serde_spanned` 0.6.9 (desktop)
- `serde_spanned` 1.1.1 (desktop)
- `serde_with` 3.23.0 (desktop)
- `serde_with_macros` 3.23.0 (desktop)
- `serialize-to-javascript` 0.1.2 (desktop)
- `serialize-to-javascript-impl` 0.1.2 (desktop)
- `servo_arc` 0.4.3 (desktop)
- `sha2` 0.10.9 (desktop,root)
- `shlex` 2.0.1 (desktop,root)
- `siphasher` 1.0.3 (desktop)
- `smallvec` 1.15.2 (root)
- `smallvec` 1.16.1 (desktop)
- `socket2` 0.6.5 (desktop)
- `softbuffer` 0.4.8 (desktop)
- `stable_deref_trait` 1.2.1 (desktop)
- `string_cache` 0.9.0 (desktop)
- `string_cache_codegen` 0.6.1 (desktop)
- `swift-rs` 1.0.8 (desktop)
- `syn` 2.0.119 (desktop,root)
- `syn` 3.0.3 (root)
- `syn` 1.0.109 (desktop)
- `syn` 3.0.5 (desktop)
- `sync_wrapper` 1.0.2 (desktop)
- `system-deps` 6.2.2 (desktop)
- `tao` 0.35.3 (desktop)
- `tao-macros` 0.1.4 (desktop)
- `target-lexicon` 0.12.16 (desktop)
- `tauri` 2.11.5 (desktop)
- `tauri-build` 2.6.3 (desktop)
- `tauri-codegen` 2.6.3 (desktop)
- `tauri-macros` 2.6.3 (desktop)
- `tauri-plugin` 2.6.3 (desktop)
- `tauri-plugin-dialog` 2.7.3 (desktop)
- `tauri-plugin-fs` 2.5.2 (desktop)
- `tauri-runtime` 2.11.3 (desktop)
- `tauri-runtime-wry` 2.11.4 (desktop)
- `tauri-utils` 2.9.3 (desktop)
- `tendril` 0.5.1 (desktop)
- `thiserror` 1.0.69 (desktop)
- `thiserror` 2.0.20 (desktop)
- `thiserror-impl` 1.0.69 (desktop)
- `thiserror-impl` 2.0.20 (desktop)
- `time` 0.3.55 (desktop)
- `time-core` 0.1.9 (desktop)
- `time-macros` 0.2.32 (desktop)
- `tinyvec` 1.13.3 (desktop)
- `toml` 0.8.2 (desktop)
- `toml` 0.9.12+spec-1.1.0 (desktop)
- `toml` 1.1.6+spec-1.1.0 (desktop)
- `toml_datetime` 0.6.3 (desktop)
- `toml_datetime` 0.7.5+spec-1.1.0 (desktop)
- `toml_datetime` 1.1.1+spec-1.1.0 (desktop)
- `toml_edit` 0.19.15 (desktop)
- `toml_edit` 0.20.2 (desktop)
- `toml_edit` 0.25.15+spec-1.1.0 (desktop)
- `toml_parser` 1.1.3+spec-1.1.0 (desktop)
- `toml_writer` 1.1.2+spec-1.1.0 (desktop)
- `tray-icon` 0.24.2 (desktop)
- `typeid` 1.0.3 (desktop)
- `typenum` 1.20.1 (desktop,root)
- `unic-char-property` 0.9.0 (desktop)
- `unic-char-range` 0.9.0 (desktop)
- `unic-common` 0.9.0 (desktop)
- `unic-ucd-ident` 0.9.0 (desktop)
- `unic-ucd-version` 0.9.0 (desktop)
- `unicode-ident` 1.0.24 (desktop,root)
- `unicode-segmentation` 1.13.3 (desktop)
- `url` 2.5.8 (desktop)
- `utf8_iter` 1.0.4 (desktop)
- `uuid` 1.24.1 (root)
- `uuid` 1.26.1 (desktop)
- `vcpkg` 0.2.15 (desktop,root)
- `version_check` 0.9.5 (desktop,root)
- `wasi` 0.11.1+wasi-snapshot-preview1 (desktop)
- `wasip2` 1.0.4+wasi-0.2.12 (desktop)
- `wasm-bindgen` 0.2.127 (root)
- `wasm-bindgen` 0.2.128 (desktop)
- `wasm-bindgen-futures` 0.4.78 (desktop)
- `wasm-bindgen-macro` 0.2.127 (root)
- `wasm-bindgen-macro` 0.2.128 (desktop)
- `wasm-bindgen-macro-support` 0.2.127 (root)
- `wasm-bindgen-macro-support` 0.2.128 (desktop)
- `wasm-bindgen-shared` 0.2.127 (root)
- `wasm-bindgen-shared` 0.2.128 (desktop)
- `wasm-streams` 0.5.0 (desktop)
- `web-sys` 0.3.105 (desktop)
- `web_atoms` 0.2.6 (desktop)
- `winapi` 0.3.9 (desktop)
- `winapi-i686-pc-windows-gnu` 0.4.0 (desktop)
- `winapi-x86_64-pc-windows-gnu` 0.4.0 (desktop)
- `window-vibrancy` 0.6.0 (desktop)
- `windows` 0.61.3 (desktop)
- `windows-collections` 0.2.0 (desktop)
- `windows-core` 0.61.2 (desktop)
- `windows-core` 0.62.2 (desktop)
- `windows-future` 0.2.1 (desktop)
- `windows-implement` 0.60.2 (desktop)
- `windows-interface` 0.59.3 (desktop)
- `windows-link` 0.1.3 (desktop)
- `windows-link` 0.2.1 (desktop)
- `windows-numerics` 0.2.0 (desktop)
- `windows-result` 0.3.4 (desktop)
- `windows-result` 0.4.1 (desktop)
- `windows-strings` 0.4.2 (desktop)
- `windows-strings` 0.5.1 (desktop)
- `windows-sys` 0.45.0 (desktop)
- `windows-sys` 0.59.0 (desktop)
- `windows-sys` 0.60.2 (desktop)
- `windows-sys` 0.61.2 (desktop)
- `windows-targets` 0.42.2 (desktop)
- `windows-targets` 0.52.6 (desktop)
- `windows-targets` 0.53.5 (desktop)
- `windows-threading` 0.1.0 (desktop)
- `windows-version` 0.1.7 (desktop)
- `windows_aarch64_gnullvm` 0.42.2 (desktop)
- `windows_aarch64_gnullvm` 0.52.6 (desktop)
- `windows_aarch64_gnullvm` 0.53.1 (desktop)
- `windows_aarch64_msvc` 0.42.2 (desktop)
- `windows_aarch64_msvc` 0.52.6 (desktop)
- `windows_aarch64_msvc` 0.53.1 (desktop)
- `windows_i686_gnu` 0.42.2 (desktop)
- `windows_i686_gnu` 0.52.6 (desktop)
- `windows_i686_gnu` 0.53.1 (desktop)
- `windows_i686_gnullvm` 0.52.6 (desktop)
- `windows_i686_gnullvm` 0.53.1 (desktop)
- `windows_i686_msvc` 0.42.2 (desktop)
- `windows_i686_msvc` 0.52.6 (desktop)
- `windows_i686_msvc` 0.53.1 (desktop)
- `windows_x86_64_gnu` 0.42.2 (desktop)
- `windows_x86_64_gnu` 0.52.6 (desktop)
- `windows_x86_64_gnu` 0.53.1 (desktop)
- `windows_x86_64_gnullvm` 0.42.2 (desktop)
- `windows_x86_64_gnullvm` 0.52.6 (desktop)
- `windows_x86_64_gnullvm` 0.53.1 (desktop)
- `windows_x86_64_msvc` 0.42.2 (desktop)
- `windows_x86_64_msvc` 0.52.6 (desktop)
- `windows_x86_64_msvc` 0.53.1 (desktop)
- `wit-bindgen` 0.57.1 (desktop)
- `wry` 0.55.1 (desktop)

