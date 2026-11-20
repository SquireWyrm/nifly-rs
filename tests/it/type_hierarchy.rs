//! Keeps the type hierarchy lists in the module docs in step with the `ni_types!` entries.
//!
//! Run with `UPDATE_HIERARCHY=1` to rewrite them after adding types.

use std::fs;

use crate::source::{Hierarchy, src_dir};

const START: &str = "<!-- type-hierarchy:start -->";
const END: &str = "<!-- type-hierarchy:end -->";

impl Hierarchy {
    /// The module's types as nested lists of links, under the types from other modules they
    /// derive from. For `objects`: its bases, and the other modules' direct subclasses that have
    /// subclasses of their own.
    fn render(&self, module: &str) -> Vec<String> {
        let in_module = |i: usize| self.classes[i].module == module;
        let mut lines = Vec::new();
        if module == "objects" {
            self.render_tree(
                module,
                0,
                0,
                &|i| {
                    in_module(i)
                        || (self.classes[i].parent.is_some_and(in_module)
                            && !self.classes[i].children.is_empty())
                },
                &mut lines,
            );
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
}

#[test]
fn module_docs_list_ni_types() {
    let src = src_dir();
    let hierarchy = Hierarchy::load(&src);
    let update = std::env::var_os("UPDATE_HIERARCHY").is_some();
    let mut stale = Vec::new();

    for module in hierarchy.modules() {
        let lines = hierarchy.render(&module);
        let expected: Vec<String> = std::iter::once(String::new())
            .chain(lines)
            .chain(std::iter::once(String::new()))
            .map(|l| {
                if l.is_empty() {
                    "//!".into()
                } else {
                    format!("//! {l}")
                }
            })
            .collect();

        let file = [
            src.join(format!("{module}.rs")),
            src.join(&module).join("mod.rs"),
        ]
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
