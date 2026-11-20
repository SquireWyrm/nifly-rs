//! Keeps the type hierarchy lists in the module docs in step with the `ni_types!` entries.
//!
//! Run with `UPDATE_HIERARCHY=1` to rewrite them after adding types.

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

const START: &str = "<!-- type-hierarchy:start -->";
const END: &str = "<!-- type-hierarchy:end -->";

struct Class {
    name: String,
    parent: Option<usize>,
    module: String,
    children: Vec<usize>,
}

struct Hierarchy {
    classes: Vec<Class>,
}

impl Hierarchy {
    fn load(src: &Path) -> Self {
        // (name, parent, module), in source order.
        let mut entries = vec![("NiObject".to_owned(), None, "objects".to_owned())];
        let objects = fs::read_to_string(src.join("objects.rs")).unwrap();
        let mut lines = objects.lines();
        while let Some(line) = lines.next() {
            if let Some(name) = line.strip_prefix("impl HasParent for ") {
                let name = name.trim_end_matches(" {").to_owned();
                let parent = lines.next().unwrap().trim();
                let parent = parent.strip_prefix("type Parent = ").unwrap().trim_end_matches(';');
                entries.push((name, Some(parent.to_owned()), "objects".to_owned()));
            }
        }

        let mut files = Vec::new();
        collect_rs(src, &mut files);
        for file in files {
            if file.ends_with("macros.rs") {
                continue;
            }
            let rel = file.strip_prefix(src).unwrap();
            let module = rel.components().next().unwrap().as_os_str().to_str().unwrap();
            let module = module.trim_end_matches(".rs").to_owned();
            let text = fs::read_to_string(&file).unwrap();
            let mut in_block = false;
            let mut entry = String::new();
            for line in text.lines() {
                let line = line.trim_end();
                if line.starts_with("ni_types! {") {
                    in_block = true;
                } else if in_block && line == "}" {
                    in_block = false;
                } else if in_block {
                    let line = line.trim();
                    if line.starts_with("///") || line.starts_with("#[") || line.is_empty() {
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
                    entries.push((name.to_owned(), Some(parent.to_owned()), module.clone()));
                }
            }
        }

        let index: BTreeMap<_, _> =
            entries.iter().enumerate().map(|(i, (name, ..))| (name.clone(), i)).collect();
        assert_eq!(index.len(), entries.len(), "duplicate type names");
        let mut classes: Vec<Class> = entries
            .iter()
            .map(|(name, parent, module)| Class {
                name: name.clone(),
                parent: parent.as_ref().map(|p| {
                    *index.get(p).unwrap_or_else(|| panic!("unknown parent {p} of {name}"))
                }),
                module: module.clone(),
                children: Vec::new(),
            })
            .collect();
        for i in 0..classes.len() {
            if let Some(p) = classes[i].parent {
                classes[p].children.push(i);
            }
        }
        Hierarchy { classes }
    }

    fn modules(&self) -> Vec<String> {
        let mut modules: Vec<String> = self.classes.iter().map(|c| c.module.clone()).collect();
        modules.sort();
        modules.dedup();
        modules
    }

    /// The module's types as nested lists of links, under the types from other modules they
    /// derive from. For `objects`: its bases, and the other modules' direct subclasses that have
    /// subclasses of their own.
    fn render(&self, module: &str) -> Vec<String> {
        let in_module = |i: usize| self.classes[i].module == module;
        let mut lines = Vec::new();
        if module == "objects" {
            self.render_tree(module, 0, 0, &|i| {
                in_module(i)
                    || (self.classes[i].parent.is_some_and(in_module)
                        && !self.classes[i].children.is_empty())
            }, &mut lines);
            return lines;
        }
        // Roots by outside parent, in order of first appearance.
        let mut parents: Vec<(usize, Vec<usize>)> = Vec::new();
        for root in (0..self.classes.len())
            .filter(|&i| in_module(i) && !self.classes[i].parent.is_some_and(in_module))
        {
            let parent = self.classes[root].parent.unwrap();
            match parents.iter_mut().find(|(p, _)| *p == parent) {
                Some((_, roots)) => roots.push(root),
                None => parents.push((parent, vec![root])),
            }
        }
        for (parent, roots) in parents {
            lines.push(format!("- {}", self.link(module, parent)));
            for root in roots {
                self.render_tree(module, root, 1, &in_module, &mut lines);
            }
        }
        lines
    }

    fn render_tree(
        &self,
        module: &str,
        class: usize,
        depth: usize,
        members: &dyn Fn(usize) -> bool,
        lines: &mut Vec<String>,
    ) {
        lines.push(format!("{}- {}", "  ".repeat(depth), self.link(module, class)));
        for &child in &self.classes[class].children {
            if members(child) {
                self.render_tree(module, child, depth + 1, members, lines);
            }
        }
    }

    fn link(&self, module: &str, class: usize) -> String {
        let c = &self.classes[class];
        if c.module == module {
            format!("[`{}`]", c.name)
        } else {
            format!("[`{1}::{0}`](crate::{1}::{0})", c.name, c.module)
        }
    }
}

fn collect_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<_> = fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_rs(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn module_docs_list_ni_types() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let hierarchy = Hierarchy::load(&src);
    let update = std::env::var_os("UPDATE_HIERARCHY").is_some();
    let mut stale = Vec::new();

    for module in hierarchy.modules() {
        let lines = hierarchy.render(&module);
        let expected: Vec<String> = std::iter::once(String::new())
            .chain(lines)
            .chain(std::iter::once(String::new()))
            .map(|l| if l.is_empty() { "//!".into() } else { format!("//! {l}") })
            .collect();

        let file = [src.join(format!("{module}.rs")), src.join(&module).join("mod.rs")]
            .into_iter()
            .find(|f| f.exists())
            .unwrap();
        let text = fs::read_to_string(&file).unwrap();
        let newline = if text.contains("\r\n") { "\r\n" } else { "\n" };
        let current: Vec<&str> = text.lines().collect();
        let start = current.iter().position(|l| *l == format!("//! {START}"));
        let end = current.iter().position(|l| *l == format!("//! {END}"));
        let (Some(start), Some(end)) = (start, end) else {
            panic!("{} has no {START} and {END} markers", file.display());
        };
        if current[start + 1..end] == expected[..] {
            continue;
        }
        if update {
            let mut new: Vec<&str> = current[..=start].to_vec();
            new.extend(expected.iter().map(String::as_str));
            new.extend(&current[end..]);
            let mut out = new.join(newline);
            if text.ends_with(newline) {
                out.push_str(newline);
            }
            fs::write(&file, out).unwrap();
        }
        stale.push(file.display().to_string());
    }

    assert!(
        update || stale.is_empty(),
        "type hierarchies out of date in {stale:?}; rerun with UPDATE_HIERARCHY=1"
    );
}
