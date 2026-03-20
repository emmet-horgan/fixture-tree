use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;

mod old;


#[derive(Debug)]
pub struct Fixture {
    fn_name: String,
    path: PathBuf,
    ext: String
}

#[derive(Debug)]
pub struct Config {
    pub(crate) allow_regexs: Vec<regex::Regex>,
    pub(crate) allow_exts: Vec<String>,
    pub(crate) ignore_paths: Vec<PathBuf>,
    pub(crate) path: PathBuf,
    pub(crate) include_ext_as_str: Vec<String>,
    pub(crate) include_ext_as_bin: Vec<String>,
    pub(crate) rel_to_manifest: bool,
    pub(crate) out_path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let out_path = std::env::var("OUT_DIR").map(|p| PathBuf::from(p)).unwrap_or(
            std::env::current_dir().unwrap()
        );
        let out_path = out_path.join("fixture_tree_autogen.rs");
        Self {
            allow_regexs: vec![],
            allow_exts: vec![],
            ignore_paths: vec![],
            include_ext_as_bin: vec![],
            include_ext_as_str: vec![],

            path: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            rel_to_manifest: false,
            out_path
        }
    }

    pub fn with_ext(mut self, ext: impl Into<String>) -> Self {
        self.allow_exts.push(ext.into());
        self
    }

    pub fn without_path(mut self, p: impl Into<PathBuf>) -> Self {
        self.ignore_paths.push(p.into());
        self
    }

    pub fn with_allow_pattern(mut self, pat: impl Into<regex::Regex>) -> Self {
        self.allow_regexs.push(pat.into());
        self
    }

    pub fn from_path(mut self, p: impl Into<PathBuf>) -> Self {
        self.path = p.into().canonicalize()
            .expect("could not make path absolute");
        self
    }

    pub fn with_ext_as_string(mut self, ext: impl Into<String>) -> Self {
        self.include_ext_as_str.push(ext.into());
        self
    }

    pub fn with_ext_as_bin(mut self, ext: impl Into<String>) -> Self {
        self.include_ext_as_bin.push(ext.into());
        self
    }

    pub fn with_output_path(mut self, p: impl Into<PathBuf>) -> Self {
        self.out_path = p.into();
        self
    }

    pub fn build(self) -> Result<FixtureTree> {
        // TODO: Check that no overlaps in ext_as_{string,bin}
        // TODO: Check that path is rel to manifest if specified
        FixtureTree::new(self)
    }
}

#[derive(Debug)]
pub struct Directory {
    directories: HashMap<String, Directory>,
    fixtures: Vec<Fixture>,
    path: PathBuf
}

impl Directory {

    pub fn from_path(p: &Path, config: &Config) -> Result<Self> {
       let mut root = Self {
            directories: HashMap::new(),
            fixtures: Vec::new(),
            path: p.to_path_buf()
       };
       root.from_path_inner(p, config)?;
       Ok(root)
    }

    fn from_path_inner(&mut self, p: &Path, config: &Config) -> Result<()> {
        if let Ok(entries) = fs::read_dir(p) {
            for entry in entries.flatten() {
                let path = entry.path();
                // Makes sense because config.path *must* but absolute and the entry path will be 
                // inside the fixtures root path meaning that this path is now relative to fixture
                // root.
                let relpath = path.strip_prefix(&config.path)?.to_path_buf();
                let ext = path
                    .extension()
                    .and_then(|s| Some(s
                            .to_string_lossy()
                            .to_string()
                            .to_lowercase()));
                
                if path.is_dir() && !config.ignore_paths.contains(&relpath) {
                    // Get directory name for module
                    let dirname = path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    // Create or get the subtree for this directory
                    let subtree = self.directories.entry(dirname.clone()).or_insert(Self {
                        directories: HashMap::new(),
                        fixtures: Vec::new(),
                        // TODO: Perhaps needs to change as we modify the dirname
                        path: p.join(&dirname),
                    });
                    subtree.from_path_inner(&path, config)?;
                } else if ext
                    .as_ref()
                    .is_some_and(|e| {
                        if config.allow_exts.is_empty() {
                            true
                        } else {
                            config.allow_exts.contains(e)
                        }
                    })
                {
                    // Generate function name from filename
                    let fn_name = path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace('-', "_")
                        .to_lowercase();

                    self.fixtures.push(Fixture { fn_name, path, ext: ext.unwrap() });
                }
            }
        }

        Ok(())
    }

    pub fn generate_code(&self, config: &Config) -> String {
        let mut buffer = String::from("// fixture-tree auto-generated fixture accessors\n\n");
        self.generate_code_inner(config, &mut buffer, 0);
        buffer
    }

    fn generate_code_inner(&self, config: &Config, buffer: &mut String, indent_level: usize) {
        let indent = "    ".repeat(indent_level);

        let path = if config.rel_to_manifest {
            self.path
                .strip_prefix(&config.path)
                .unwrap()
                
        } else {
            &self.path
        };
        let path = path
            .components()
            .map(|c| c.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/");

        buffer.push_str(&format!(
            "{}pub fn path() -> &'static std::path::Path {{\n",
            indent
        ));
        if config.rel_to_manifest {
            buffer.push_str(&format!(
                "{}    std::path::Path::new(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{}\"))\n",
                indent, path
            ));
        } else {
            buffer.push_str(&format!(
                "{}    std::path::Path::new(\"{}\")\n",
                indent, path
            ));
        }
        buffer.push_str(&format!("{}}}\n\n", indent));

        // Generate functions for fixtures at this level
        for f in &self.fixtures {
            // ensure unix style path
            let path = if config.rel_to_manifest {
                f.path
                    .strip_prefix(&config.path)
                    .unwrap()
                
            } else {
                &f.path
            };
            let path = path
                .components()
                .map(|c| c.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/");
            if config.include_ext_as_str.contains(&f.ext) {
                if config.rel_to_manifest {
                    buffer.push_str(&format!(
r#"{}pub fn {}() -> (&'static std::path::Path, &'static str) {{
{}    (std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/{}")), include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/{}")))
{}}}
                
"#,
                        indent, f.fn_name, indent, path, path, indent
                    ));
                } else {
                    buffer.push_str(&format!(
r#"{}pub fn {}() -> (&'static std::path::Path, &'static str) {{
{}    (std::path::Path::new("{}"), include_str!("{}"))
{}}}
                
"#,
                        indent, f.fn_name, indent, path, path, indent
                    ));
                }
            } else if config.include_ext_as_bin.contains(&f.ext){
                if config.rel_to_manifest {
                    buffer.push_str(&format!(
r#"{}pub fn {}() -> (&'static std::path::Path, &'static [u8]) {{
{}    (std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/{}")), include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/{}")))
{}}}

"#,
                        indent, f.fn_name, indent, path, path, indent
                    ));
                } else {
                    buffer.push_str(&format!(
r#"{}pub fn {}() -> (&'static std::path::Path, &'static [u8]) {{
{}    (std::path::Path::new("{}"), include_bytes!("{}"))
{}}}

"#,
                        indent, f.fn_name, indent, path, path, indent
                    ));
                }
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

    pub fn is_empty(&self) -> bool {
        self.fixtures.is_empty() && self.directories.values().all(|d| d.is_empty()) 
    }
}

#[derive(Debug)]
pub struct FixtureTree {
    root: Directory,
    config: Config
}

impl FixtureTree {
    pub fn new(config: Config) -> Result<Self> {
        let root = Directory::from_path(&config.path, &config)?;
        Ok(Self { root, config })
    }

    pub fn generate_fixtures(&self) -> Result<()> {
        let fixtures = self.root.generate_code(&self.config);
        fs::write(&self.config.out_path, fixtures)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_functionality() {
        let ft = Config::new()
            .from_path("/home/ehorgan/weaver/crates/weaver-test-utils/fixtures")
            .with_output_path("./fixtures.rs")
            .with_ext("json")
            .with_ext_as_string("json")
            .build()
            .unwrap();

        ft.generate_fixtures()
            .unwrap();
    }
}