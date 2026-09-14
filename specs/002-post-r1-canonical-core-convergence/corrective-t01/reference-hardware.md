# Reference hardware pin — T00-02 (canonical plan §27)

"T00-02 freezes exact measurement hardware; T05-02 repeats on each supported native profile." This is the development-profile pin only. It is not a performance-gate PASS claim — no benchmark has been run against it yet; this record exists so that when T01-07 (and later P05 tasks) do measure, the hardware is fixed in advance rather than reported after the fact.

## Development profile (this session's host)

| Field | Value | Source |
|---|---|---|
| CPU | 13th Gen Intel(R) Core(TM) i5-13420H | `Get-CimInstance Win32_Processor` |
| Physical cores / logical processors | 8 / 12 | `Get-CimInstance Win32_Processor` |
| RAM | 16,106 MB (≈15.7 GiB) | `systeminfo` |
| Storage | WD PC SN740 SDDPNQD-512G-1127, NVMe SSD | `Get-PhysicalDisk` |
| System drive filesystem | NTFS | `Get-CimInstance Win32_LogicalDisk` |
| Free space on system drive at intake | 6.4 GiB of 200 GiB | `Get-CimInstance Win32_LogicalDisk` — **noted as a real constraint**: dataset-L (10 GiB canonical payload) fixtures will not fit without freeing space first; flagged for whoever runs L-scale tests, not assumed away |
| OS | Windows 11 Home, build 10.0.26200 | `systeminfo` |
| Toolchain | `rustc 1.97.1 (8bab26f4f 2026-07-14)`, `cargo 1.97.1 (c980f4866 2026-06-30)` | `rustc --version` / `cargo --version` |
| Git | `2.55.0.windows.5` | `git --version` |

Meets the canonical plan §27 reference minimum ("four physical CPU cores, 8 GiB RAM, local SSD, no concurrent benchmark load") with margin on CPU/RAM; SSD confirmed NVMe. "No concurrent benchmark load" is a per-run condition to be asserted at actual benchmark time (T01-07 onward), not a static hardware fact — this record does not claim it holds for any specific future run.

## Native profiles still required (T05-02, out of this addendum's scope)

| Profile | Status |
|---|---|
| Linux / ext4 | Not available in this environment; deferred to `T05-02` per canonical plan, consistent with `D6` being explicitly out of `T01-07`'s completion condition |
| macOS / APFS | Not available in this environment; deferred to `T05-02` |
| Windows / NTFS | Available — this is the profile T01-07's D1-D5 pass will actually use |

Recording an unavailable profile as unavailable, rather than omitting it or claiming a pass, follows canonical plan §34 (T00-01's own UX-behavior clause, generalized here): "Record available native hosts; absence is an execution prerequisite, not a false pass."
