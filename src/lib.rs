#![allow(clippy::needless_doctest_main)]
//! # fixture-tree
//!
//! Generate Rust source code that mirrors a filesystem directory as a module tree,
//! providing zero-cost accessors for file paths and contents at compile time.
//!
//! `fixture-tree` is intended for use from **build scripts** (`build.rs`). It walks a
//! directory of fixture files (test data, configs, model weights, etc.) and emits a
//! `.rs` file where every directory becomes a `mod` and every file becomes a function
//! returning its path and, optionally, its contents via `include_str!` or
//! `include_bytes!`.
//!
//! This can be particularly useful if you find yourself using a lot of static files for testing
//! specific use-cases. This allows you to use code linting to easily traverse your fixture
//! file structure and will result in a compilation error if you mix around file paths.
//!
//! ## Quick start
//!
//! ```rust,no_run
//! // build.rs
//! fn main() {
//!     fixture_tree::Config::new()
//!         .from_path("fixtures")           // relative to CARGO_MANIFEST_DIR
//!         .with_ext("json")                // only include .json files
//!         .with_ext_as_string("json")       // embed contents via include_str!
//!         .build()
//!         .unwrap()
//!         .generate_fixtures()
//!         .unwrap();
//! }
//! ```
//!
//! Then in your library or test code:
//!
//! ```rust,ignore
//! include!(concat!(env!("OUT_DIR"), "/fixture_tree_autogen.rs"));
//!
//! #[test]
//! fn read_fixture() {
//!     let (path, contents) = configs::pass::basic();
//!     assert!(path.exists());
//!     assert!(contents.contains("ok"));
//! }
//! ```
//!
//! ## Path handling
//!
//! When the source directory is under `CARGO_MANIFEST_DIR` (the typical case),
//! generated paths use `concat!(env!("CARGO_MANIFEST_DIR"), "/...")` so they
//! resolve correctly on any machine. If the source directory is outside the
//! manifest (e.g. a system temp directory), absolute paths are emitted instead.
//!
//! ## Filtering
//!
//! Files can be filtered in two ways:
//!
//! - **By extension** — [`Config::with_ext`] / [`Config::with_exts`] restrict
//!   which file extensions are walked. An empty list means "all extensions".
//! - **By regex** *(requires the `regex` feature)* -
//!   [`Config::with_allow_pattern`] / [`Config::with_allow_patterns`]
//!   match against the file's path relative to the source root. When at least one
//!   regex is configured a file must match *any* of them to be included.
//!
//! Entire subtrees can be excluded with [`Config::without_path`] /
//! [`Config::without_paths`], which compare against the directory's relative path.
//!
//! ## Generated code shape
//!
//! For each directory a `path()` function is emitted. For each matched file a
//! function named after the file stem (lowercased, dashes replaced with
//! underscores) is emitted. Files registered as string extensions return
//! `(&'static Path, &'static str)`, binary extensions return
//! `(&'static Path, &'static [u8])` and files not registered as binary or strings
//! just return a `&'static Path`.
//! Empty directories are pruned from the
//! output.

use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

/// Errors that can occur when building or generating a [`FixtureTree`].
#[derive(Debug)]
pub enum Error {
    /// An extension was registered as both string and binary.
    ExtConflict(String),
    /// A path could not be made relative to the source root.
    StripPrefix(std::path::StripPrefixError),
    /// An I/O error occurred while reading the source directory or writing the
    /// output file.
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ExtConflict(ext) => write!(
                f,
                "'{}' cannot be registered as both string and binary",
                ext
            ),
            Error::StripPrefix(e) => write!(f, "failed to compute relative path: {}", e),
            Error::Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ExtConflict(_) => None,
            Error::StripPrefix(e) => Some(e),
            Error::Io(e) => Some(e),
        }
    }
}

impl From<std::path::StripPrefixError> for Error {
    fn from(e: std::path::StripPrefixError) -> Self {
        Error::StripPrefix(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

/// A specialised [`Result`] type for fixture-tree operations.
pub type Result<T> = std::result::Result<T, Error>;

/// A discovered fixture file.
///
/// Holds the generated function name, the resolved filesystem path, and the
/// lowercased file extension.
#[derive(Debug)]
pub struct Fixture {
    fn_name: String,
    path: PathBuf,
    ext: String,
}

/// Internal representation of the source directory.
///
/// If the directory is under `CARGO_MANIFEST_DIR` it stores a relative path
/// and sets `rel_to_manifest = true` so that generated code uses
/// `concat!(env!("CARGO_MANIFEST_DIR"), "...")` rather than absolute paths.
#[derive(Debug)]
struct SourceDirectory {
    p: PathBuf,
    rel_to_manifest: bool,
}

impl SourceDirectory {
    fn new(p: impl Into<PathBuf>) -> Self {
        let p = p.into();
        let mut p = p
            .canonicalize()
            .expect("could not make source path absolute");
        let rel_to_manifest = match std::env::var("CARGO_MANIFEST_DIR") {
            Ok(c) => match p.strip_prefix(&c) {
                Ok(stripped) => {
                    p = stripped.to_path_buf();
                    true
                }
                _ => false,
            },
            _ => false,
        };
        Self { p, rel_to_manifest }
    }

    fn is_rel(&self) -> bool {
        self.rel_to_manifest
    }

    fn path(&self) -> &Path {
        &self.p
    }
}

impl Default for SourceDirectory {
    fn default() -> Self {
        let cur = std::env::current_dir().expect("could not find the current directory");
        Self::new(cur)
    }
}

/// Builder for configuring a [`FixtureTree`].
///
/// Construct with [`Config::new`], chain builder methods to set options, and
/// finalise with [`Config::build`].
///
/// # Examples
///
/// ```rust,no_run
/// let tree = fixture_tree::Config::new()
///     .from_path("test_data")
///     .with_exts(["json", "toml"])
///     .with_exts_as_string(["json", "toml"])
///     .without_path("scratch")
///     .build()
///     .unwrap();
/// tree.generate_fixtures().unwrap();
/// ```
#[derive(Debug)]
pub struct Config {
    #[cfg(feature = "regex")]
    pub(crate) allow_regexs: Vec<regex::Regex>,
    pub(crate) allow_exts: Vec<String>,
    pub(crate) ignore_paths: Vec<PathBuf>,
    pub(crate) source: SourceDirectory,
    pub(crate) include_ext_as_str: Vec<String>,
    pub(crate) include_ext_as_bin: Vec<String>,
    pub(crate) out_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    /// Create a new config with default settings.
    ///
    /// Defaults:
    /// - **source directory**: current working directory
    /// - **output path**: `$OUT_DIR/fixture_tree_autogen.rs` (falls back to cwd
    ///   when `OUT_DIR` is not set)
    /// - **filters**: none (all files included)
    pub fn new() -> Self {
        let out_path = std::env::var("OUT_DIR")
            .map(PathBuf::from)
            .unwrap_or(std::env::current_dir().unwrap());
        let out_path = out_path.join("fixture_tree_autogen.rs");

        Self {
            #[cfg(feature = "regex")]
            allow_regexs: vec![],
            allow_exts: vec![],
            ignore_paths: vec![],
            include_ext_as_bin: vec![],
            include_ext_as_str: vec![],
            source: Default::default(),
            out_path,
        }
    }

    /// Only include files whose extension matches `ext` (case-insensitive).
    ///
    /// Can be called multiple times to allow several extensions. If no
    /// extension filter is set, all files are included.
    pub fn with_ext(mut self, ext: impl Into<String>) -> Self {
        self.allow_exts.push(ext.into());
        self
    }

    /// Batch version of [`Config::with_ext`].
    ///
    /// ```rust,no_run
    /// # let config = fixture_tree::Config::new();
    /// config.with_exts(["json", "toml", "yaml"]);
    /// ```
    pub fn with_exts(mut self, exts: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.allow_exts.extend(exts.into_iter().map(|x| x.into()));
        self
    }

    /// Exclude an entire directory subtree by its path relative to the source
    /// root.
    ///
    /// ```rust,no_run
    /// # let config = fixture_tree::Config::new();
    /// config.without_path("snapshots");
    /// ```
    pub fn without_path(mut self, p: impl Into<PathBuf>) -> Self {
        self.ignore_paths.push(p.into());
        self
    }

    /// Batch version of [`Config::without_path`].
    pub fn without_paths(mut self, paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> Self {
        self.ignore_paths
            .extend(paths.into_iter().map(|x| x.into()));
        self
    }

    /// Only include files whose relative path matches the given regex.
    ///
    /// Multiple patterns are combined with OR semantics — a file is included if
    /// it matches *any* pattern. The pattern is matched against the file's path
    /// relative to the source root (e.g. `"configs/pass/basic.json"`).
    ///
    /// Requires the `regex` feature.
    ///
    /// # Panics
    ///
    /// Panics if the pattern string is not a valid regex.
    #[cfg(feature = "regex")]
    pub fn with_allow_pattern(mut self, pat: impl Into<String>) -> Self {
        self.allow_regexs
            .push(regex::Regex::new(&pat.into()).expect("could not create regex"));
        self
    }

    /// Batch version of [`Config::with_allow_pattern`].
    ///
    /// Requires the `regex` feature.
    ///
    /// # Panics
    ///
    /// Panics if any pattern string is not a valid regex.
    #[cfg(feature = "regex")]
    pub fn with_allow_patterns(
        mut self,
        pats: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let pats = pats
            .into_iter()
            .map(|x| regex::Regex::new(&x.into()).expect("could not create regex"));
        self.allow_regexs.extend(pats);
        self
    }

    /// Set the root directory to scan for fixture files.
    ///
    /// The path is canonicalised and, if it falls under `CARGO_MANIFEST_DIR`,
    /// generated code will reference it relative to the manifest via
    /// `env!("CARGO_MANIFEST_DIR")`.
    ///
    /// # Panics
    ///
    /// Panics if the path does not exist or cannot be canonicalised.
    pub fn from_path(mut self, p: impl Into<PathBuf>) -> Self {
        let source = SourceDirectory::new(p);
        self.source = source;
        self
    }

    /// Register a file extension whose contents should be embedded as a
    /// `&'static str` via `include_str!`.
    ///
    /// The generated accessor for matching files returns
    /// `(&'static Path, &'static str)`.
    pub fn with_ext_as_string(mut self, ext: impl Into<String>) -> Self {
        self.include_ext_as_str.push(ext.into());
        self
    }

    /// Batch version of [`Config::with_ext_as_string`].
    pub fn with_exts_as_string(
        mut self,
        exts: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.include_ext_as_str
            .extend(exts.into_iter().map(|x| x.into()));
        self
    }

    /// Register a file extension whose contents should be embedded as a
    /// `&'static [u8]` via `include_bytes!`.
    ///
    /// The generated accessor for matching files returns
    /// `(&'static Path, &'static [u8])`.
    pub fn with_ext_as_bin(mut self, ext: impl Into<String>) -> Self {
        self.include_ext_as_bin.push(ext.into());
        self
    }

    /// Batch version of [`Config::with_ext_as_bin`].
    pub fn with_exts_as_bin(mut self, exts: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.include_ext_as_bin
            .extend(exts.into_iter().map(|x| x.into()));
        self
    }

    /// Override the path where the generated Rust source file is written.
    ///
    /// Defaults to `$OUT_DIR/fixture_tree_autogen.rs`.
    pub fn with_output_path(mut self, p: impl Into<PathBuf>) -> Self {
        self.out_path = p.into();
        self
    }

    /// Validate the configuration and build a [`FixtureTree`].
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - An extension is registered as both string and binary.
    /// - The source directory cannot be read.
    pub fn build(self) -> Result<FixtureTree> {
        for ext in &self.include_ext_as_str {
            if self.include_ext_as_bin.contains(ext) {
                return Err(Error::ExtConflict(ext.clone()));
            }
        }
        FixtureTree::new(self)
    }
}

/// In-memory representation of a scanned directory.
///
/// Each `Directory` holds its child directories (as nested `Directory` values)
/// and the [`Fixture`] files found at this level. Used internally to build
/// the module tree before code generation.
#[derive(Debug)]
pub struct Directory {
    directories: BTreeMap<String, Directory>,
    fixtures: Vec<Fixture>,
    path: PathBuf,
}

impl Directory {
    /// Scan a directory and build the tree according to `config`.
    pub fn from_path(p: &Path, config: &Config) -> Result<Self> {
        let mut root = Self {
            directories: BTreeMap::new(),
            fixtures: Vec::new(),
            path: p.to_path_buf(),
        };
        root.from_path_inner(p, config)?;
        Ok(root)
    }

    #[allow(clippy::wrong_self_convention)]
    fn from_path_inner(&mut self, p: &Path, config: &Config) -> Result<()> {
        if let Ok(entries) = fs::read_dir(p) {
            for entry in entries.flatten() {
                let path = entry.path();
                let relpath = path.strip_prefix(config.source.path())?.to_path_buf();
                let ext = path
                    .extension()
                    .map(|s| s.to_string_lossy().to_string().to_lowercase());

                if path.is_dir() && !config.ignore_paths.contains(&relpath) {
                    // Get directory name for module
                    let dirname = path.file_name().unwrap().to_str().unwrap().to_string();
                    // Create or get the subtree for this directory
                    let subtree = self.directories.entry(dirname.clone()).or_insert(Self {
                        directories: BTreeMap::new(),
                        fixtures: Vec::new(),
                        path: p.join(&dirname),
                    });
                    subtree.from_path_inner(&path, config)?;
                } else if ext.as_ref().is_some_and(|e| {
                    if config.allow_exts.is_empty() {
                        true
                    } else {
                        config.allow_exts.contains(e)
                    }
                }) {
                    // If regex filters are configured, the relative path must
                    // match at least one of them to be included.
                    #[cfg(feature = "regex")]
                    if !config.allow_regexs.is_empty() {
                        let relpath_str = relpath.to_string_lossy();
                        if !config.allow_regexs.iter().any(|r| r.is_match(&relpath_str)) {
                            continue;
                        }
                    }

                    // Generate function name from filename
                    let fn_name = path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace('-', "_")
                        .to_lowercase();

                    self.fixtures.push(Fixture {
                        fn_name,
                        path,
                        ext: ext.unwrap(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Render the full generated Rust source for this directory tree.
    pub fn generate_code(&self, config: &Config) -> String {
        let mut buffer = String::from("// fixture-tree auto-generated fixture accessors\n\n");
        self.generate_code_inner(config, &mut buffer, 0);
        buffer
    }

    fn generate_code_inner(&self, config: &Config, buffer: &mut String, indent_level: usize) {
        let indent = "    ".repeat(indent_level);

        let path = self.path.to_string_lossy().replace('\\', "/");

        if config.source.is_rel() {
            rel_mod_path(buffer, &path, &indent);
        } else {
            non_rel_mod_path(buffer, &path, &indent);
        }

        // Generate functions for fixtures at this level
        for f in &self.fixtures {
            let path = f.path.to_string_lossy().replace('\\', "/");

            if config.include_ext_as_str.contains(&f.ext) {
                if config.source.is_rel() {
                    rel_as_string_file(buffer, &f.fn_name, &path, &indent);
                } else {
                    non_rel_as_string_file(buffer, &f.fn_name, &path, &indent);
                }
            } else if config.include_ext_as_bin.contains(&f.ext) {
                if config.source.is_rel() {
                    rel_as_bin_file(buffer, &f.fn_name, &path, &indent);
                } else {
                    non_rel_as_bin_file(buffer, &f.fn_name, &path, &indent);
                }
            } else if config.source.is_rel() {
                rel_file_path(buffer, &f.fn_name, &path, &indent);
            } else {
                non_rel_file_path(buffer, &f.fn_name, &path, &indent);
            }
        }

        // Generate nested modules
        for (module_name, subtree) in &self.directories {
            if subtree.is_empty() {
                continue;
            }
            buffer.push_str(&format!("{}pub mod {} {{\n\n", indent, module_name));
            subtree.generate_code_inner(config, buffer, indent_level + 1);
            buffer.push_str(&format!("{}}}\n\n", indent));
        }
    }

    /// Returns `true` when this directory (and all descendants) contain no
    /// fixtures. Used to prune empty modules from the generated output.
    pub fn is_empty(&self) -> bool {
        self.fixtures.is_empty() && self.directories.values().all(|d| d.is_empty())
    }
}

/// The top-level handle returned by [`Config::build`].
///
/// Call [`FixtureTree::generate_fixtures`] to write the generated Rust source
/// file to disk.
#[derive(Debug)]
pub struct FixtureTree {
    root: Directory,
    config: Config,
}

impl FixtureTree {
    /// Create a new `FixtureTree` by scanning the configured source directory.
    pub fn new(config: Config) -> Result<Self> {
        let root = Directory::from_path(config.source.path(), &config)?;
        Ok(Self { root, config })
    }

    /// Generate the Rust source file and write it to the configured output
    /// path.
    ///
    /// # Errors
    ///
    /// Returns an error if the output file cannot be written.
    pub fn generate_fixtures(&self) -> Result<()> {
        let fixtures = self.root.generate_code(&self.config);
        fs::write(&self.config.out_path, fixtures)?;
        Ok(())
    }
}

fn non_rel_mod_path(buffer: &mut String, path: &str, indent: &str) {
    write!(
        buffer,
        r#"{indent}pub fn path() -> &'static std::path::Path {{
{indent}    std::path::Path::new("{path}")
{indent}}}

"#
    )
    .unwrap();
}

fn rel_mod_path(buffer: &mut String, path: &str, indent: &str) {
    write!(
        buffer,
        "{indent}pub fn path() -> &'static std::path::Path {{\n\
{indent}    std::path::Path::new(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{path}\"))\n\
{indent}}}\n\n"
    )
    .unwrap();
}

fn non_rel_file_path(buffer: &mut String, fn_name: &str, path: &str, indent: &str) {
    write!(
        buffer,
        r#"{indent}pub fn {fn_name}() -> &'static std::path::Path {{
{indent}    std::path::Path::new("{path}")
{indent}}}

"#
    )
    .unwrap();
}

fn rel_file_path(buffer: &mut String, fn_name: &str, path: &str, indent: &str) {
    write!(
        buffer,
        r#"{indent}pub fn {fn_name}() -> &'static std::path::Path {{
{indent}    std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/{path}"))
{indent}}}

"#
    )
    .unwrap();
}

fn rel_as_string_file(buffer: &mut String, fn_name: &str, path: &str, indent: &str) {
    write!(buffer,
r#"{indent}pub fn {fn_name}() -> (&'static std::path::Path, &'static str) {{
{indent}    (std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/{path}")), include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/{path}")))
{indent}}}

"#
    ).unwrap();
}

fn non_rel_as_string_file(buffer: &mut String, fn_name: &str, path: &str, indent: &str) {
    write!(
        buffer,
        r#"{indent}pub fn {fn_name}() -> (&'static std::path::Path, &'static str) {{
{indent}    (std::path::Path::new("{path}"), include_str!("{path}"))
{indent}}}

"#
    )
    .unwrap();
}

fn rel_as_bin_file(buffer: &mut String, fn_name: &str, path: &str, indent: &str) {
    write!(buffer,
r#"{indent}pub fn {fn_name}() -> (&'static std::path::Path, &'static [u8]) {{
{indent}    (std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/{path}")), include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/{path}")))
{indent}}}

"#
    ).unwrap();
}

fn non_rel_as_bin_file(buffer: &mut String, fn_name: &str, path: &str, indent: &str) {
    write!(
        buffer,
        r#"{indent}pub fn {fn_name}() -> (&'static std::path::Path, &'static [u8]) {{
{indent}    (std::path::Path::new("{path}"), include_bytes!("{path}"))
{indent}}}

"#
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    /// Lay out a sufficiently complex fixture tree under `root`:
    fn create_fixture_tree(root: &Path) {
        let dirs = [
            "",
            "models",
            "configs",
            "configs/pass",
            "configs/fail",
            "ignored",
        ];
        for d in &dirs {
            fs::create_dir_all(root.join(d)).unwrap();
        }

        let text_files = [
            ("alpha.json", r#"{"a": 1}"#),
            ("beta.json", r#"{"b": 2}"#),
            ("delta.txt", "plain text"),
            ("models/linear.json", r#"{"type": "linear"}"#),
            ("models/conv.json", r#"{"type": "conv"}"#),
            ("configs/pass/basic.json", r#"{"ok": true}"#),
            (
                "configs/pass/advanced.json",
                r#"{"ok": true, "level": "advanced"}"#,
            ),
            ("configs/fail/bad_config.json", r#"{"ok": false}"#),
            ("ignored/should_skip.json", r#"{"skip": true}"#),
        ];
        for (name, content) in &text_files {
            fs::write(root.join(name), content).unwrap();
        }

        // Binary files
        fs::write(root.join("gamma.bin"), &[0xDE, 0xAD, 0xBE, 0xEF]).unwrap();
        fs::write(root.join("models/weights.bin"), &[0x01, 0x02, 0x03]).unwrap();
    }

    /// Build a fixture tree and return the generated code as a String.
    fn generate_to_string(config: Config) -> String {
        let out = config.out_path.clone();
        let ft = config.build().unwrap();
        ft.generate_fixtures().unwrap();
        fs::read_to_string(&out).unwrap()
    }

    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    /// Create a temp dir outside `CARGO_MANIFEST_DIR` and populate it.
    fn setup_tempdir() -> TempDir {
        let tmp = TempDir::new().unwrap();
        create_fixture_tree(tmp.path());
        tmp
    }

    /// Create a uniquely-named dir inside the project (under CARGO_MANIFEST_DIR)
    /// and populate it. Each call gets its own directory so tests can run in
    /// parallel without interfering.
    struct InManifestDir(PathBuf);

    impl InManifestDir {
        fn new() -> Self {
            let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
            let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let test_root = manifest.join("test_fixtures");
            let dir = test_root.join(format!("auto_{id}"));
            if dir.exists() {
                fs::remove_dir_all(&dir).unwrap();
            }
            create_fixture_tree(&dir);
            Self(dir)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for InManifestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    // in-manifest tests (rel_to_manifest = true)

    #[test]
    fn manual_review_all() {
        let dir = InManifestDir::new();
        let out = dir.path().parent().unwrap().join("manual_review_all.rs");

        Config::new()
            .from_path(dir.path())
            .with_output_path(out.clone())
            .with_exts_as_string(["json", "txt"])
            .with_exts_as_bin(["bin"])
            .build()
            .unwrap()
            .generate_fixtures()
            .unwrap();
    }

    #[test]
    fn manual_review_only_stringy() {
        let dir = InManifestDir::new();
        let out = dir
            .path()
            .parent()
            .unwrap()
            .join("manual_review_stringy.rs");

        Config::new()
            .from_path(dir.path())
            .with_output_path(out.clone())
            .with_exts_as_string(["json", "txt"])
            .build()
            .unwrap()
            .generate_fixtures()
            .unwrap();
    }

    #[test]
    fn in_manifest_generates_all_json_files() {
        let dir = InManifestDir::new();
        let out = dir.path().join("_test_out.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(dir.path())
                .with_output_path(&out)
                .with_ext("json")
                .with_ext_as_string("json"),
        );

        // root-level json files present
        assert!(code.contains("pub fn alpha()"), "alpha.json missing");
        assert!(code.contains("pub fn beta()"), "beta.json missing");

        // nested files present
        assert!(
            code.contains("pub fn linear()"),
            "models/linear.json missing"
        );
        assert!(code.contains("pub fn conv()"), "models/conv.json missing");
        assert!(
            code.contains("pub fn basic()"),
            "configs/pass/basic.json missing"
        );
        assert!(
            code.contains("pub fn advanced()"),
            "configs/pass/advanced.json missing"
        );
        assert!(
            code.contains("pub fn bad_config()"),
            "configs/fail/bad_config.json missing"
        );

        // non-json files excluded
        assert!(
            !code.contains("pub fn gamma()"),
            "gamma.bin should be excluded by ext filter"
        );
        assert!(
            !code.contains("pub fn delta()"),
            "delta.txt should be excluded by ext filter"
        );
        assert!(
            !code.contains("pub fn weights()"),
            "weights.bin should be excluded by ext filter"
        );

        // uses CARGO_MANIFEST_DIR (relative)
        assert!(
            code.contains("env!(\"CARGO_MANIFEST_DIR\")"),
            "should use CARGO_MANIFEST_DIR"
        );
        assert!(
            !code.contains("\"/home"),
            "should not contain absolute paths"
        );
    }

    #[test]
    fn in_manifest_nested_modules_structure() {
        let dir = InManifestDir::new();
        let out = dir.path().join("_test_structure.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(dir.path())
                .with_output_path(&out)
                .with_ext("json")
                .with_ext_as_string("json"),
        );

        assert!(code.contains("pub mod models {"), "models module missing");
        assert!(code.contains("pub mod configs {"), "configs module missing");
        assert!(code.contains("pub mod pass {"), "pass module missing");
        assert!(code.contains("pub mod fail {"), "fail module missing");
        assert!(code.contains("pub mod ignored {"), "ignored module missing");
    }

    #[test]
    fn in_manifest_path_fn_per_module() {
        let dir = InManifestDir::new();
        let out = dir.path().join("_test_paths.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(dir.path())
                .with_output_path(&out)
                .with_ext("json")
                .with_ext_as_string("json"),
        );

        // Every module gets a path() function
        let path_count = code.matches("pub fn path()").count();
        // root + models + configs + pass + fail + ignored = 6
        assert!(
            path_count >= 6,
            "expected at least 6 path() fns, got {path_count}"
        );
    }

    #[test]
    fn in_manifest_ext_as_string_returns_tuple() {
        let dir = InManifestDir::new();
        let out = dir.path().join("_test_str.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(dir.path())
                .with_output_path(&out)
                .with_ext("json")
                .with_ext_as_string("json"),
        );

        assert!(
            code.contains("(&'static std::path::Path, &'static str)"),
            "string fixtures should return (Path, str) tuple"
        );
        assert!(
            code.contains("include_str!"),
            "should use include_str! for string fixtures"
        );
    }

    #[test]
    fn in_manifest_ext_as_bin_returns_bytes() {
        let dir = InManifestDir::new();
        let out = dir.path().join("_test_bin.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(dir.path())
                .with_output_path(&out)
                .with_ext("bin")
                .with_ext_as_bin("bin"),
        );

        assert!(code.contains("pub fn gamma()"), "gamma.bin missing");
        assert!(
            code.contains("pub fn weights()"),
            "models/weights.bin missing"
        );
        assert!(
            code.contains("(&'static std::path::Path, &'static [u8])"),
            "binary fixtures should return (Path, [u8]) tuple"
        );
        assert!(
            code.contains("include_bytes!"),
            "should use include_bytes! for binary fixtures"
        );
    }

    #[test]
    fn in_manifest_multiple_exts() {
        let dir = InManifestDir::new();
        let out = dir.path().join("_test_multi_ext.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(dir.path())
                .with_output_path(&out)
                .with_ext("json")
                .with_ext("bin")
                .with_ext_as_string("json")
                .with_ext_as_bin("bin"),
        );

        // Both types present
        assert!(code.contains("pub fn alpha()"), "json fixture missing");
        assert!(code.contains("pub fn gamma()"), "bin fixture missing");
        assert!(code.contains("include_str!"), "missing include_str");
        assert!(code.contains("include_bytes!"), "missing include_bytes");
    }

    #[test]
    fn in_manifest_no_ext_filter_includes_everything() {
        let dir = InManifestDir::new();
        let out = dir.path().join("_test_no_ext.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(dir.path())
                .with_output_path(&out)
                .with_ext_as_string("json")
                .with_ext_as_string("txt")
                .with_ext_as_bin("bin"),
        );

        // All extensions should be walked when allow_exts is empty
        assert!(code.contains("pub fn alpha()"), "json missing");
        assert!(code.contains("pub fn gamma()"), "bin missing");
        assert!(code.contains("pub fn delta()"), "txt missing");
    }

    #[test]
    fn in_manifest_ignore_paths() {
        let dir = InManifestDir::new();
        let out = dir.path().join("_test_ignore.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(dir.path())
                .with_output_path(&out)
                .with_ext("json")
                .with_ext_as_string("json")
                .without_path(PathBuf::from("ignored")),
        );

        assert!(
            !code.contains("pub fn should_skip()"),
            "ignored dir should be excluded"
        );
        assert!(
            code.contains("pub fn alpha()"),
            "non-ignored files should remain"
        );
    }

    #[test]
    fn in_manifest_ignore_multiple_paths() {
        let dir = InManifestDir::new();
        let out = dir.path().join("_test_ignore_multi.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(dir.path())
                .with_output_path(&out)
                .with_ext("json")
                .with_ext_as_string("json")
                .without_paths(vec![PathBuf::from("ignored"), PathBuf::from("configs")]),
        );

        assert!(!code.contains("pub fn should_skip()"), "ignored/ excluded");
        assert!(!code.contains("pub fn basic()"), "configs/ excluded");
        assert!(!code.contains("pub fn bad_config()"), "configs/ excluded");
        assert!(code.contains("pub fn alpha()"), "root files remain");
        assert!(code.contains("pub fn linear()"), "models/ remain");
    }

    #[test]
    #[cfg(feature = "regex")]
    fn in_manifest_regex_filter_filename() {
        let dir = InManifestDir::new();
        let out = dir.path().join("_test_regex_name.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(dir.path())
                .with_output_path(&out)
                .with_ext("json")
                .with_ext_as_string("json")
                .with_allow_pattern(r"alpha|beta"),
        );

        assert!(code.contains("pub fn alpha()"), "alpha should match");
        assert!(code.contains("pub fn beta()"), "beta should match");
        assert!(!code.contains("pub fn linear()"), "linear should not match");
        assert!(!code.contains("pub fn basic()"), "basic should not match");
    }

    #[test]
    #[cfg(feature = "regex")]
    fn in_manifest_regex_filter_path_component() {
        let dir = InManifestDir::new();
        let out = dir.path().join("_test_regex_path.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(dir.path())
                .with_output_path(&out)
                .with_ext("json")
                .with_ext_as_string("json")
                .with_allow_pattern(r"^configs/pass/"),
        );

        assert!(
            code.contains("pub fn basic()"),
            "configs/pass/basic.json should match"
        );
        assert!(
            code.contains("pub fn advanced()"),
            "configs/pass/advanced.json should match"
        );
        assert!(
            !code.contains("pub fn bad_config()"),
            "configs/fail/ should not match"
        );
        assert!(
            !code.contains("pub fn alpha()"),
            "root files should not match"
        );
    }

    #[test]
    #[cfg(feature = "regex")]
    fn in_manifest_multiple_regex_patterns_or() {
        let dir = InManifestDir::new();
        let out = dir.path().join("_test_regex_multi.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(dir.path())
                .with_output_path(&out)
                .with_ext("json")
                .with_ext_as_string("json")
                .with_allow_patterns(vec![r"^alpha", r"^models/"]),
        );

        assert!(
            code.contains("pub fn alpha()"),
            "alpha should match first pattern"
        );
        assert!(
            code.contains("pub fn linear()"),
            "models/ should match second pattern"
        );
        assert!(
            code.contains("pub fn conv()"),
            "models/ should match second pattern"
        );
        assert!(
            !code.contains("pub fn beta()"),
            "beta should not match either pattern"
        );
    }

    #[test]
    fn in_manifest_empty_dirs_omitted() {
        let dir = InManifestDir::new();
        let out = dir.path().join("_test_empty.rs");
        // Only allow .txt, the only .txt file is at the root, so all subdirs
        // with only .json/.bin become empty and should be pruned.
        let code = generate_to_string(
            Config::new()
                .from_path(dir.path())
                .with_output_path(&out)
                .with_ext("txt")
                .with_ext_as_string("txt"),
        );

        assert!(
            code.contains("pub fn delta()"),
            "delta.txt should be present"
        );
        assert!(
            !code.contains("pub mod models {"),
            "empty models module should be omitted"
        );
        assert!(
            !code.contains("pub mod configs {"),
            "empty configs module should be omitted"
        );
    }

    #[test]
    fn in_manifest_fn_name_sanitisation() {
        let dir = InManifestDir::new();
        // Create a file with dashes in the name
        fs::write(dir.path().join("my-dashed-name.json"), "{}").unwrap();
        let out = dir.path().join("_test_sanitise.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(dir.path())
                .with_output_path(&out)
                .with_ext("json")
                .with_ext_as_string("json"),
        );

        assert!(
            code.contains("pub fn my_dashed_name()"),
            "dashes should be replaced with underscores"
        );
        assert!(
            !code.contains("pub fn my-dashed-name()"),
            "raw dashes should not appear in fn names"
        );
    }

    #[test]
    fn in_manifest_ext_overlap_rejected() {
        let dir = InManifestDir::new();
        let out = dir.path().join("_test_overlap.rs");
        let result = Config::new()
            .from_path(dir.path())
            .with_output_path(&out)
            .with_ext("json")
            .with_ext_as_string("json")
            .with_ext_as_bin("json")
            .build();

        assert!(result.is_err(), "overlapping string/bin ext should fail");
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("json"),
            "error should mention the offending ext"
        );
    }

    // temp-dir tests (rel_to_manifest = false)

    #[test]
    fn tempdir_generates_absolute_paths() {
        let tmp = setup_tempdir();
        let out = tmp.path().join("_test_out.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(tmp.path())
                .with_output_path(&out)
                .with_ext("json")
                .with_ext_as_string("json"),
        );

        // Should NOT reference CARGO_MANIFEST_DIR
        assert!(
            !code.contains("env!(\"CARGO_MANIFEST_DIR\")"),
            "temp-dir output should not use CARGO_MANIFEST_DIR"
        );

        // Should contain absolute paths from the tempdir
        let tmp_str = tmp.path().to_string_lossy();
        assert!(
            code.contains(tmp_str.as_ref()),
            "output should contain the temp dir absolute path"
        );
    }

    #[test]
    fn tempdir_generates_all_json_files() {
        let tmp = setup_tempdir();
        let out = tmp.path().join("_test_all.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(tmp.path())
                .with_output_path(&out)
                .with_ext("json")
                .with_ext_as_string("json"),
        );

        assert!(code.contains("pub fn alpha()"), "alpha missing");
        assert!(code.contains("pub fn beta()"), "beta missing");
        assert!(code.contains("pub fn linear()"), "linear missing");
        assert!(code.contains("pub fn basic()"), "basic missing");
        assert!(code.contains("pub fn bad_config()"), "bad_config missing");
        assert!(
            !code.contains("pub fn gamma()"),
            "gamma.bin should be excluded"
        );
    }

    #[test]
    fn tempdir_ext_as_bin() {
        let tmp = setup_tempdir();
        let out = tmp.path().join("_test_bin.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(tmp.path())
                .with_output_path(&out)
                .with_ext("bin")
                .with_ext_as_bin("bin"),
        );

        assert!(code.contains("pub fn gamma()"), "gamma.bin missing");
        assert!(code.contains("pub fn weights()"), "weights.bin missing");
        assert!(code.contains("include_bytes!"), "should use include_bytes!");
        assert!(
            !code.contains("include_str!"),
            "should not use include_str!"
        );
    }

    #[test]
    fn tempdir_ignore_paths() {
        let tmp = setup_tempdir();
        let out = tmp.path().join("_test_ign.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(tmp.path())
                .with_output_path(&out)
                .with_ext("json")
                .with_ext_as_string("json")
                .without_path(PathBuf::from("ignored"))
                .without_path(PathBuf::from("models")),
        );

        assert!(!code.contains("pub fn should_skip()"), "ignored/ excluded");
        assert!(!code.contains("pub fn linear()"), "models/ excluded");
        assert!(code.contains("pub fn alpha()"), "root files remain");
    }

    #[test]
    #[cfg(feature = "regex")]
    fn tempdir_regex_filter() {
        let tmp = setup_tempdir();
        let out = tmp.path().join("_test_regex.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(tmp.path())
                .with_output_path(&out)
                .with_ext("json")
                .with_ext_as_string("json")
                .with_allow_pattern(r"configs/"),
        );

        assert!(
            code.contains("pub fn basic()"),
            "configs/ files should match"
        );
        assert!(
            code.contains("pub fn bad_config()"),
            "configs/ files should match"
        );
        assert!(
            !code.contains("pub fn alpha()"),
            "root files should not match"
        );
        assert!(
            !code.contains("pub fn linear()"),
            "models/ should not match"
        );
    }

    #[test]
    fn tempdir_nested_modules() {
        let tmp = setup_tempdir();
        let out = tmp.path().join("_test_mods.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(tmp.path())
                .with_output_path(&out)
                .with_ext("json")
                .with_ext_as_string("json"),
        );

        assert!(code.contains("pub mod models {"), "models module");
        assert!(code.contains("pub mod configs {"), "configs module");
        assert!(code.contains("pub mod pass {"), "pass submodule");
        assert!(code.contains("pub mod fail {"), "fail submodule");
    }

    #[test]
    fn tempdir_empty_modules_pruned() {
        let tmp = setup_tempdir();
        let out = tmp.path().join("_test_prune.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(tmp.path())
                .with_output_path(&out)
                .with_ext("txt")
                .with_ext_as_string("txt"),
        );

        assert!(code.contains("pub fn delta()"), "delta.txt present");
        assert!(!code.contains("pub mod models {"), "empty dir pruned");
    }

    #[test]
    fn tempdir_mixed_string_and_bin() {
        let tmp = setup_tempdir();
        let out = tmp.path().join("_test_mixed.rs");
        let code = generate_to_string(
            Config::new()
                .from_path(tmp.path())
                .with_output_path(&out)
                .with_ext("json")
                .with_ext("bin")
                .with_ext_as_string("json")
                .with_ext_as_bin("bin"),
        );

        // String
        assert!(code.contains("include_str!"), "json uses include_str!");
        // Bin
        assert!(code.contains("include_bytes!"), "bin uses include_bytes!");
        // Both types present
        assert!(code.contains("pub fn alpha()"), "json fixture");
        assert!(code.contains("pub fn gamma()"), "bin fixture");
    }

    #[test]
    fn tempdir_ext_overlap_rejected() {
        let tmp = setup_tempdir();
        let out = tmp.path().join("_test_overlap.rs");
        let result = Config::new()
            .from_path(tmp.path())
            .with_output_path(&out)
            .with_ext("json")
            .with_ext_as_string("json")
            .with_ext_as_bin("json")
            .build();

        assert!(result.is_err(), "overlapping ext should be rejected");
    }
}
