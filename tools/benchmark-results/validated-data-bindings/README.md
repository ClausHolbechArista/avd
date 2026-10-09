# Validated-data binding comparison

Measured locally on 2026-10-09. Recommendation: use the native descriptor-backed models for the
AVD validated-data API. They materially improve startup, private memory, and the operations most
relevant to presence checks and collection navigation. These results do not establish an AVD
whole-build speedup.

## Implementations

The baseline is the checkpointed Python forwarding implementation: generated properties and
collection methods over Rust-owned handles. Checkpoints are `f85f8f0` in pyavd-utils and
`03f9698a3c` in AVD. Neither implementation materializes complete dictionaries/lists on access.

The native implementation exposes `pyavd._rust._validated_data` directly from the extension and
re-exports it lazily as `from pyavd import _validated_data`. A generated
`python_data_views!` catalog defines all 1,458 Python-visible model identities, including contextual
primary-key items. Rust creates the named Python types and installs shared native descriptors,
collection operations, and iterators. There are no generated Python forwarding functions in this
path. Relaxed subtrees remain `OpaqueData`, intentionally without Python descendant accessors.

The borrowed, validation-mode-parameterized Rust views remain intact. They are not themselves
`#[pyclass]` types: Python objects own archive handles and cannot retain those Rust lifetimes.
The native binding implementation uses the same registry's field slots after checked archive
opening. Scalar materialization, null/undefined behavior, and opaque payload preservation are
unchanged. Cached Python child-type references participate in Python cyclic garbage collection.

The comparison was made while both implementations were available. Native bindings are now the
sole implementation: the generated Python source, its generator, and the forwarding-only Rust
classes have been removed. The generated `.pyi` remains the static typing contract. The private
native module is loaded through `_rust` on first access; importing `pyavd` alone does not initialize
the extension. The original comparison measurements confirm no generated Python source was loaded
on the native path. Measurements below are from the comparison build, before this cleanup.

## Controlled runtime measurements

Same Python 3.13.12 interpreter, release builds, CPU affinity `[8]`, 200,000 iterations for the
accessor cases, one warmup and seven recorded samples per case. Table values are medians.
The read workload instead makes 100 passes through 64 devices and four tenants, performing
scalar, presence, boolean, list, indexed-list, hybrid-list, and opaque truthiness operations.

| Operation | Python forwarding | Native descriptors | Python/native time ratio |
| --- | ---: | ---: | ---: |
| Small-integer property | 12.28 ms | 9.64 ms | 1.27 |
| Undefined/null/bool checks | 116.45 ms | 25.34 ms | 4.60 |
| Nested device lookup and property | 108.24 ms | 88.45 ms | 1.22 |
| Positional list navigation | 66.39 ms | 59.34 ms | 1.12 |
| Indexed device lookup and property | 65.89 ms | 41.34 ms | 1.59 |
| Synthetic read workload | 14.01 ms | 7.86 ms | 1.78 |
| Open and check one archive | 1.404 ms | 1.384 ms | 1.01 |
| Fresh-process import, 15 processes | 19.70 ms | 9.99 ms | 1.97 |

Opening is essentially unchanged: both paths perform the same schema/archive compatibility and
typed-root checks. Import means a fresh interpreter with warm filesystem and bytecode caches,
not cold-disk I/O. The small-integer test uses CPython's cached integer range; positional list
navigation and the read workload also materialize string values.

Earlier short, unpinned scalar samples reversed direction between runs, so they were not used
to decide scalar performance. The longer pinned scalar samples range from 11.77–12.55 ms for
Python and 9.57–9.77 ms for native. Raw pinned samples and process snapshots are in
[baseline-pinned.json](baseline-pinned.json) and [native-pinned.json](native-pinned.json).
Other unpinned measurements remain in `/tmp/validated-bindings-benchmark/`; their synthetic
workload speedup varied between roughly 1.5 and 3.3. The controlled comparison above is the
preferred result, not the largest observed speedup.

All case checksums match. Both implementations publish byte-identical 39,912-byte data archives:
`5a503755ab2aeda7cf5168e8c1387a7a9b0034ff8f27425e202227896eff1e22`.

## Multi-process memory

Four independently launched processes simultaneously open the same schema/data files and each
retain 20,000 device child views. Measurements are Linux `smaps_rollup` snapshots, not peak
allocation counts. PSS apportions shared pages; RSS alone would double-count them.

| Aggregate across four processes | Python forwarding | Native descriptors |
| --- | ---: | ---: |
| PSS | 83.70 MiB | 53.34 MiB |
| Private dirty memory | 73.85 MiB | 43.10 MiB |

Native PSS is about 36% lower; private dirty memory is about 42% lower. Both implementations use
the same mmap-backed archive sharing. This difference primarily concerns per-process Python
binding objects, not a change to the data arena's sharing behavior. The small fixture is not
representative of the total data size of a large production host.

Raw snapshots: [Python](multiprocess-baseline-round1.json), [native](multiprocess-native.json).

## Build and size costs

Rust 1.95.0, Cargo release profile, no LTO. Stripping was performed only on temporary artifact
copies. The native binary still contains the baseline helpers needed for the comparison.

| Artifact | Python forwarding | Native descriptors |
| --- | ---: | ---: |
| Release extension | 8,445,704 bytes | 9,060,272 bytes |
| Stripped extension | 6,330,080 bytes | 6,923,328 bytes |
| Generated runtime Python source | 1,116,212 bytes | Not required |
| Representative compressed payload | 1,585,092 bytes | 1,611,522 bytes |

The compressed estimate is a ZIP_DEFLATED level-6 archive containing the stripped extension plus
the runtime Python source for baseline, versus the stripped extension alone for native. It omits
the unchanged stub and all other unchanged package files. It is not an actual wheel build.
Native adds about 26 KiB (1.7%) to that compressed payload, while reducing the corresponding
uncompressed installed payload by about 0.50 MiB. The stripped extension alone grows about 9%.
Details: [artifact-summary.json](artifact-summary.json).

The native extension-only rebuild with warm dependencies took 29.54 seconds and about 2.05 GiB
peak RSS; a no-op rebuild took 0.08 seconds. The first baseline build took 43.88 seconds and about
2.04 GiB peak RSS, but included cold dependency work. Those times are **not a controlled build-time
comparison**, and do not establish a compilation speedup. The native prototype did not require
thousands of separate per-field PyO3 implementations.

## Correctness and reproduction

Checks passed:

- The eight existing AVD tests unchanged through each implementation.
- Three additional AVD cases covering every generated model's public field set, collection
  edge cases, explicit defaults, immutability, and retained iterators through both implementations.
- Two native Rust/Python integration tests covering archive retention and collection of cached
  self-referencing Python types.
- Eleven generator tests, including unchanged registry/stub/source output when adding the native
  catalog.
- Scoped Ruff, rustfmt, and native-crate Clippy checks.

Run from the AVD repository with its virtualenv, a matching installed release extension, and
`PYTHONPATH=python-avd`:

```shell
.venv/bin/python tools/benchmark-validated-data-bindings.py \
  --label baseline --cpu 8 --iterations 200000 \
  --directory /tmp/validated-bindings-benchmark
.venv/bin/python tools/benchmark-validated-data-bindings.py \
  --label native --cpu 8 --iterations 200000 \
  --directory /tmp/validated-bindings-benchmark
```

CPU 8 must be replaced with an available CPU on other machines. For an exact comparison, use
the checkpointed baseline sources/extension for the first command and the native sources/extension
for the second, using the same benchmark script. The script resolves the same package API in each
checkout. Python forwarding classes are no longer available in the current checkout. Saved release artifacts and build/test logs
are in `/tmp/validated-bindings-benchmark/`.

Builds used `/tmp/validated-data-local-cargo.toml` to patch the experimental Rust dependencies
to the current pyavd-utils checkout. The checked-in Git pins have not been advanced or pushed.
The AVD lockfile was restored after local patched builds. Apart from the initial checkpoints,
the implementation and measurements remain uncommitted.

## Native-only cleanup verification

The follow-up cleanup removes the generated Python forwarding module and the generic Python view
handles. The canonical private API is now backed directly by the native module at
`pyavd._validated_data`. The historical comparison above and its raw JSON remain unchanged; this
section records checks against the cleaned-up native-only implementation.

Focused checks passed with a freshly built release extension:

- `test_rust.py`: 8 passed.
- `test_native_validated_data.py`: 4 passed.
- `test_lazy_imports.py`: 5 passed.
- Ruff checks/formatting and Cargo rustfmt checks passed.
- The release build used the local pyavd-utils patches; Cargo.lock was restored afterward.

The final native-only benchmark ran on CPU 8 with 200,000 iterations. The generated Python module
was not loaded, and all checksums matched:

| Operation | Median |
| --- | ---: |
| Small-integer property | 9.80 ms |
| Undefined/null/bool checks | 25.44 ms |
| Nested device lookup and property | 91.41 ms |
| Positional list navigation | 62.61 ms |
| Indexed device lookup and property | 47.17 ms |
| Synthetic read workload | 9.15 ms |
| Open and check one archive | 1.629 ms |
| Fresh-process import | 13.01 ms |

The release extension is 9,045,192 bytes. A temporary copy stripped with
`strip --strip-unneeded` is 6,902,552 bytes. The benchmark archive is 39,912 bytes with the same
SHA-256 recorded above. [native-only.json](native-only.json) contains raw samples, checksums,
process memory, and runtime metadata. Build and test logs are in `/tmp/validated-bindings-benchmark/`.
