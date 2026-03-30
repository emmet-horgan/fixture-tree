# fixture-tree

Generate Rust source code that mirrors a filesystem directory as a module tree,
providing zero-cost accessors for file paths and contents at compile time.

`fixture-tree` is intended for use from **build scripts** (`build.rs`). It walks
a directory of fixture files (test data, configs, model weights, etc.) and emits
a `.rs` file where every directory becomes a `mod` and every file becomes a
function returning its path and, optionally, its contents via `include_str!` or
`include_bytes!`.

This is particularly useful when you rely on many static files for testing
specific use-cases. It gives you code-completion for your fixture paths and turns
stale or mistyped paths into compile errors.

## Quick start

Add `fixture-tree` as a **build** dependency:

```toml
[build-dependencies]
fixture-tree = "0.1"
```

Create a `build.rs`:

```rust
fn main() {
    fixture_tree::Config::new()
        .from_path("fixtures")           // relative to CARGO_MANIFEST_DIR
        .with_ext("json")                // only include .json files
        .with_ext_as_string("json")      // embed contents via include_str!
        .build()
        .unwrap()
        .generate_fixtures()
        .unwrap();
}
```

Then include the generated file in your library or test code:

```rust
include!(concat!(env!("OUT_DIR"), "/fixture_tree_autogen.rs"));

#[test]
fn read_fixture() {
    let (path, contents) = configs::pass::basic();
    assert!(path.exists());
    assert!(contents.contains("ok"));
}
```

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `regex` | no | Enables `with_allow_pattern` / `with_allow_patterns` for regex-based file filtering. |

Enable it in your `Cargo.toml`:

```toml
[build-dependencies]
fixture-tree = { version = "0.1", features = ["regex"] }
```

## Path handling

When the source directory is under `CARGO_MANIFEST_DIR` (the typical case),
generated paths use `concat!(env!("CARGO_MANIFEST_DIR"), "/...")` so they resolve
correctly on any machine. If the source directory is outside the manifest (e.g. a
system temp directory), absolute paths are emitted instead.

## Filtering

Files can be filtered in several ways:

- **By extension** — `with_ext` / `with_exts` restrict which file extensions are
  walked. An empty list means "all extensions".
- **By regex** *(requires the `regex` feature)* — `with_allow_pattern` /
  `with_allow_patterns` match against the file's path relative to the source
  root. When at least one regex is configured a file must match *any* of them to
  be included.
- **By path exclusion** — `without_path` / `without_paths` exclude entire
  directory subtrees by their path relative to the source root.

## Generated code shape

For each directory a `path()` function is emitted. For each matched file a
function named after the file stem (lowercased, dashes replaced with underscores)
is emitted:

| Registration | Return type |
|-------------|-------------|
| `with_ext_as_string` | `(&'static Path, &'static str)` |
| `with_ext_as_bin` | `(&'static Path, &'static [u8])` |
| *(neither)* | `&'static Path` |

Empty directories are pruned from the output.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
