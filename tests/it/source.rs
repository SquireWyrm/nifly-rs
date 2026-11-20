//! Reads the block types from the library's source, for the tests that keep generated docs in
//! step with it.

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

pub struct Class {
    pub name: String,
    pub parent: Option<usize>,
    /// The top-level module that defines it.
    pub module: String,
    pub children: Vec<usize>,
    /// Whether its definition has a doc comment of its own.
    pub documented: bool,
}

/// Every block type: the `ni_types!` entries and the hand-written bases in `objects.rs`.
pub struct Hierarchy {
    pub classes: Vec<Class>,
    index: BTreeMap<String, usize>,
}

impl Hierarchy {
    pub fn load(src: &Path) -> Self {
        // (name, parent, module, documented), in source order.
        let mut entries = vec![("NiObject".to_owned(), None, "objects".to_owned(), true)];
        let objects = fs::read_to_string(src.join("objects.rs")).unwrap();
        let mut lines = objects.lines();
        while let Some(line) = lines.next() {
            if let Some(name) = line.strip_prefix("impl HasParent for ") {
                let name = name.trim_end_matches(" {").to_owned();
                let parent = lines.next().unwrap().trim();
                let parent = parent
                    .strip_prefix("type Parent = ")
                    .unwrap()
                    .trim_end_matches(';');
                entries.push((name, Some(parent.to_owned()), "objects".to_owned(), true));
            }
        }

        for file in rust_files(src) {
            if file.ends_with("macros.rs") {
                continue;
            }
            let module = module_of(src, &file);
            let text = fs::read_to_string(&file).unwrap();
            let mut in_block = false;
            let mut entry = String::new();
            let mut documented = false;
            for line in text.lines() {
                let line = line.trim_end();
                if line.starts_with("ni_types! {") {
                    in_block = true;
                } else if in_block && line == "}" {
                    in_block = false;
                } else if in_block {
                    let line = line.trim();
                    if line.starts_with("///") {
                        documented = true;
                        continue;
                    }
                    if line.starts_with("#[") || line.is_empty() {
                        continue;
                    }
                    // Entries may wrap.
                    entry.push_str(line);
                    entry.push(' ');
                    if !line.ends_with(';') {
                        continue;
                    }
                    let line = std::mem::take(&mut entry);
                    // `Name(Ffi): Parent => upcast, downcast;`
                    let (name, rest) = line.split_once('(').unwrap_or_else(|| panic!("{line}"));
                    let (_, rest) = rest.split_once("):").unwrap_or_else(|| panic!("{line}"));
                    let (parent, _) = rest.split_once("=>").unwrap_or_else(|| panic!("{line}"));
                    let parent = parent.trim().rsplit("::").next().unwrap();
                    entries.push((
                        name.to_owned(),
                        Some(parent.to_owned()),
                        module.clone(),
                        std::mem::take(&mut documented),
                    ));
                }
            }
        }

        let index: BTreeMap<_, _> = entries
            .iter()
            .enumerate()
            .map(|(i, (name, ..))| (name.clone(), i))
            .collect();
        assert_eq!(index.len(), entries.len(), "duplicate type names");
        let mut classes: Vec<Class> = entries
            .iter()
            .map(|(name, parent, module, documented)| Class {
                name: name.clone(),
                parent: parent.as_ref().map(|p| {
                    *index
                        .get(p)
                        .unwrap_or_else(|| panic!("unknown parent {p} of {name}"))
                }),
                module: module.clone(),
                children: Vec::new(),
                documented: *documented,
            })
            .collect();
        for i in 0..classes.len() {
            if let Some(p) = classes[i].parent {
                classes[p].children.push(i);
            }
        }
        Hierarchy { classes, index }
    }

    /// The class named `name`.
    pub fn get(&self, name: &str) -> Option<usize> {
        self.index.get(name).copied()
    }

    /// `class` and its bases, nearest first.
    pub fn ancestors(&self, class: usize) -> impl Iterator<Item = usize> + '_ {
        std::iter::successors(Some(class), |&c| self.classes[c].parent)
    }

    pub fn modules(&self) -> Vec<String> {
        let mut modules: Vec<String> = self.classes.iter().map(|c| c.module.clone()).collect();
        modules.sort();
        modules.dedup();
        modules
    }

    /// A doc link to `class` from `module`'s docs.
    pub fn link(&self, module: &str, class: usize) -> String {
        let c = &self.classes[class];
        link(module, &c.module, &c.name)
    }
}

/// A doc link to `name` in `item_module`, from `module`'s docs. Items from other modules show
/// their module, which also keeps the link text from resolving to the same item (rustdoc's
/// `redundant_explicit_links`).
pub fn link(module: &str, item_module: &str, name: &str) -> String {
    if item_module == module {
        format!("[`{name}`]")
    } else {
        format!("[`{item_module}::{name}`](crate::{item_module}::{name})")
    }
}

pub fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// The top-level module of a file under `src`.
pub fn module_of(src: &Path, file: &Path) -> String {
    let rel = file.strip_prefix(src).unwrap();
    let first = rel
        .components()
        .next()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap();
    first.trim_end_matches(".rs").to_owned()
}

/// Every `.rs` file under `dir`, sorted.
pub fn rust_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    collect_rs(dir, &mut out);
    out
}

fn collect_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_rs(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}
