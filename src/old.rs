use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const ALLOWED_FIXTURE_TYPES: &[&str] = &["json", "tflite", "onnx", "bin", "raw"];

const STRING_FILE_TYPES: &[&str] = &["json"];

fn main() {
    println!("cargo:rerun-if-changed=fixtures/");

    let fixtures_dir = Path::new("fixtures");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_fixtures.rs");

    let mut fixture_tree = FixtureTree::new(fixtures_dir.to_path_buf());

    // Recursively find all JSON files and build the tree
    collect_fixtures(fixtures_dir, &mut fixture_tree);

    let mut code = String::from("// Auto-generated fixture accessors\n\n");
    code.push_str(&generate_code(&fixture_tree, 0));

    fs::write(dest_path, code).unwrap();
}

#[derive(Debug)]
struct FixtureTree {
    modules: HashMap<String, FixtureTree>,
    fixtures: Vec<(String, PathBuf, String)>,
    path: PathBuf,
    
}

impl FixtureTree {
    fn new(base: PathBuf) -> Self {
        Self {
            modules: HashMap::new(),
            fixtures: Vec::new(),
            path: base,
        }
    }

    fn is_empty(&self) -> bool {
        self.fixtures.is_empty() && self.modules.values().all(|m| m.is_empty())
    }
}

fn collect_fixtures(current_dir: &Path, tree: &mut FixtureTree) {
    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.to_lowercase());
            if path.is_dir() {
                // Get directory name for module
                let dir_name = path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace('-', "_")
                    .to_lowercase();

                // Create or get the subtree for this directory
                let subtree = tree.modules.entry(dir_name.clone()).or_insert(FixtureTree {
                    modules: HashMap::new(),
                    fixtures: Vec::new(),
                    path: current_dir.join(&dir_name),
                });

                // Recursively process subdirectory
                collect_fixtures(&path, subtree);
            } else if ext
                .as_ref()
                .is_some_and(|ext| ALLOWED_FIXTURE_TYPES.contains(&ext.as_str()))
            {
                // Generate function name from filename
                let fn_name = path
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace('-', "_")
                    .to_lowercase();

                tree.fixtures.push((fn_name, path, ext.unwrap()));
            }
        }
    }
}

fn generate_code(tree: &FixtureTree, indent_level: usize) -> String {
    let indent = "    ".repeat(indent_level);
    let mut code = String::new();

    let path = tree
        .path
        .components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/");
    code.push_str(&format!(
        "{}pub fn path() -> &'static std::path::Path {{\n",
        indent
    ));
    code.push_str(&format!(
        "{}    std::path::Path::new(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{}\"))\n",
        indent, path
    ));
    code.push_str(&format!("{}}}\n\n", indent));

    // Generate functions for fixtures at this level
    for (fn_name, path, ext) in &tree.fixtures {
        // ensure unix style path
        let path = path
            .components()
            .map(|c| c.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/");
        if STRING_FILE_TYPES.contains(&ext.as_str()) {
            code.push_str(&format!(
r#"{}pub fn {}() -> (&'static std::path::Path, &'static str) {{
{}    (std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/{}")), include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/{}")))
{}}}

"#,
                indent, fn_name, indent, path, path, indent
            ));
        } else {
            code.push_str(&format!(
r#"{}pub fn {}() -> (&'static std::path::Path, &'static [u8]) {{
{}    (std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/{}")), include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/{}")))
{}}}

"#,
                indent, fn_name, indent, path, path, indent
            ));
        }
    }

    // Generate nested modules
    for (module_name, subtree) in &tree.modules {
        if subtree.is_empty() {
            continue;
        }
        code.push_str(&format!("{}pub mod {} {{\n\n", indent, module_name));
        code.push_str(&generate_code(subtree, indent_level + 1));
        code.push_str(&format!("{}}}\n\n", indent));
    }

    code
}
