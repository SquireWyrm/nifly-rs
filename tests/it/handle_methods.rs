//! Keeps each block type's "Handle methods" section, `docs/handle_methods/<Type>.md`, in step
//! with the `View` and `Edit` methods that work on it.
//!
//! Run with `UPDATE_HANDLE_METHODS=1` to rewrite the files after adding types or methods. A new
//! type's file must exist before the crate compiles, so create it empty first.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use crate::source::{Hierarchy, module_of, rust_files, src_dir};

/// Which types a handle method works on.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Bound {
    /// `impl View<'a, X>`: `X` only; its subtypes after `upcast`.
    Exact(String),
    /// `where T: IsA<X, I>`: `X` and its subtypes.
    IsA(String),
    /// `where T: Trait`: the types `Trait` is implemented for.
    Trait(String),
}

struct Method {
    handle: &'static str,
    name: String,
    bound: Bound,
}

struct Source {
    hierarchy: Hierarchy,
    /// In source order.
    methods: Vec<Method>,
    /// The other methods of handle impls, by handle.
    private: Vec<(&'static str, String)>,
    /// Trait name to its module and implementors.
    traits: BTreeMap<String, (String, BTreeSet<String>)>,
}

impl Source {
    fn load(src: &Path) -> Self {
        let hierarchy = Hierarchy::load(src);
        let mut methods = Vec::new();
        let mut private = Vec::new();
        let mut traits: BTreeMap<String, (String, BTreeSet<String>)> = BTreeMap::new();
        let mut implementors: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

        for file in rust_files(src) {
            let module = module_of(src, &file);
            let text = fs::read_to_string(&file).unwrap().replace('\r', "");
            for line in text.lines() {
                if let Some(rest) = line.strip_prefix("pub trait ") {
                    let name = rest
                        .split(|c: char| !c.is_alphanumeric() && c != '_')
                        .next();
                    traits.insert(name.unwrap().to_owned(), (module.clone(), BTreeSet::new()));
                }
                // `impl Trait for X {`
                if let Some(rest) = line.strip_prefix("impl ")
                    && let Some((tr, ty)) = rest.split_once(" for ")
                    && !tr.contains(['<', '$'])
                {
                    let ty = ty.trim_end_matches(" {}").trim_end_matches(" {");
                    implementors
                        .entry(tr.to_owned())
                        .or_default()
                        .insert(ty.to_owned());
                }
            }
            // `impl_marker! { Trait: A B }` and `chained_modifier! { Next: A B }`.
            for (call, fixed) in [
                ("impl_marker! {", None),
                ("chained_modifier! {", Some("ChainedModifier")),
            ] {
                for (at, _) in text.match_indices(call) {
                    let body = &text[at + call.len()..];
                    let body = &body[..body.find('}').unwrap()];
                    let (head, types) = body.split_once(':').unwrap();
                    let tr = fixed.unwrap_or(head.trim());
                    let set = implementors.entry(tr.to_owned()).or_default();
                    set.extend(types.split_whitespace().map(str::to_owned));
                }
            }
            if !file.ends_with("handle.rs") {
                parse_handle_methods(&file, &text, &mut methods, &mut private);
            }
        }
        for (tr, (_, set)) in &mut traits {
            if let Some(found) = implementors.remove(tr) {
                *set = found;
            }
        }
        Source {
            hierarchy,
            methods,
            private,
            traits,
        }
    }

    /// The groups of methods that work on `class`: the type itself, the marker traits it
    /// implements, then its bases, nearest first. Each has its summary (plain HTML, where doc
    /// links don't work) and, for an exact base type's methods, a note on reaching them.
    fn groups(&self, class: usize) -> Vec<Group<'_>> {
        let h = &self.hierarchy;
        let name = &h.classes[class].name;
        let mut groups = Vec::new();
        let mut push = |summary: String, note: Option<String>, pick: &dyn Fn(&Bound) -> bool| {
            let methods: Vec<&Method> = self.methods.iter().filter(|m| pick(&m.bound)).collect();
            if !methods.is_empty() {
                groups.push(Group {
                    summary,
                    note,
                    methods,
                });
            }
        };
        let from = |name: &str| format!("From <code>{name}</code>");

        let own = |b: &Bound| matches!(b, Bound::Exact(x) | Bound::IsA(x) if x == name);
        push(from(name), None, &own);

        let mut seen = BTreeSet::new();
        for m in &self.methods {
            if let Bound::Trait(tr) = &m.bound
                && self.traits[tr].1.contains(name)
                && seen.insert(tr.clone())
            {
                push(from(tr), None, &|b| *b == Bound::Trait(tr.clone()));
            }
        }

        let module = &h.classes[class].module;
        for base in h.ancestors(class).skip(1) {
            let base_name = &h.classes[base].name;
            push(from(base_name), None, &|b| {
                *b == Bound::IsA(base_name.clone())
            });
            let note = format!(
                "These are on handles of exactly {}, so [`upcast`](crate::handle::View::upcast) to it first.",
                h.link(module, base)
            );
            push(
                format!("{}, after <code>upcast</code>", from(base_name)),
                Some(note),
                &|b| *b == Bound::Exact(base_name.clone()),
            );
        }
        groups
    }

    /// The section: a collapsible block per group, the type's own open, listing each handle's
    /// methods.
    fn render(&self, class: usize) -> String {
        let groups = self.groups(class);
        if groups.is_empty() {
            return String::new();
        }
        let mut out = String::from(
            "
# Handle methods
",
        );
        for (i, group) in groups.iter().enumerate() {
            let open = if i == 0
                && group.summary
                    == format!("From <code>{}</code>", self.hierarchy.classes[class].name)
            {
                " open"
            } else {
                ""
            };
            out.push_str(&format!(
                "
<details{open}>
<summary>{}</summary>

",
                group.summary
            ));
            if let Some(note) = &group.note {
                out.push_str(&format!(
                    "{note}

"
                ));
            }
            for handle in ["View", "Edit"] {
                let methods: Vec<&&Method> = group
                    .methods
                    .iter()
                    .filter(|m| m.handle == handle)
                    .collect();
                if methods.is_empty() {
                    continue;
                }
                out.push_str(&format!(
                    "- `{handle}`
"
                ));
                for m in methods {
                    out.push_str(&format!(
                        "  - [`{0}`](crate::handle::{handle}::{0})
",
                        m.name
                    ));
                }
            }
            out.push_str(
                "
</details>
",
            );
        }
        out
    }
}

struct Group<'a> {
    summary: String,
    note: Option<String>,
    methods: Vec<&'a Method>,
}

/// Collects the `pub fn`s of the `impl … View<…>` and `impl … Edit<…>` blocks in `text`, and
/// the names of their other methods.
fn parse_handle_methods(
    file: &Path,
    text: &str,
    out: &mut Vec<Method>,
    private: &mut Vec<(&'static str, String)>,
) {
    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        i += 1;
        let Some(handle) = ["View", "Edit"]
            .into_iter()
            .find(|h| line.starts_with("impl") && line.contains(&format!(" {h}<")))
        else {
            continue;
        };
        // `impl<'a> View<'a, X> {` or `impl<T: NiType> Edit<'_, T> {`
        let args = line.split_once(&format!(" {handle}<")).unwrap().1;
        let target = args
            .split_once('>')
            .unwrap()
            .0
            .rsplit(',')
            .next()
            .unwrap()
            .trim();
        while i < lines.len() && lines[i] != "}" {
            let Some(rest) = lines[i].strip_prefix("    pub fn ") else {
                let other = lines[i].trim_start_matches("    pub(crate) ");
                let other = other.trim_start_matches("    pub(super) ");
                if let Some(rest) = other.strip_prefix("    fn ").or(other.strip_prefix("fn ")) {
                    private.push((handle, rest.split(['(', '<']).next().unwrap().to_owned()));
                }
                i += 1;
                continue;
            };
            let name = rest.split(['(', '<']).next().unwrap().to_owned();
            let mut signature = String::new();
            while !lines[i].trim_end().ends_with('{') {
                signature.push_str(lines[i]);
                i += 1;
            }
            signature.push_str(lines[i]);
            let bound = if target != "T" {
                Bound::Exact(target.to_owned())
            } else {
                let predicate = signature
                    .split_once("where")
                    .and_then(|(_, w)| w.split_once("T:"))
                    .map(|(_, p)| p.trim())
                    .unwrap_or_else(|| {
                        panic!("{}: {handle}::{name} has no `T:` bound", file.display())
                    });
                match predicate.strip_prefix("IsA<") {
                    Some(base) => Bound::IsA(base.split(',').next().unwrap().trim().to_owned()),
                    None => Bound::Trait(
                        predicate
                            .split(|c: char| !c.is_alphanumeric() && c != '_')
                            .next()
                            .unwrap()
                            .to_owned(),
                    ),
                }
            };
            out.push(Method {
                handle,
                name,
                bound,
            });
            i += 1;
        }
    }
}

#[test]
fn type_docs_list_handle_methods() {
    let source = Source::load(&src_dir());
    let h = &source.hierarchy;

    // Every bound names a known type or a marker trait with known implementors.
    for m in &source.methods {
        match &m.bound {
            Bound::Exact(x) | Bound::IsA(x) => {
                assert!(
                    h.get(x).is_some(),
                    "{}::{}: unknown type {x}",
                    m.handle,
                    m.name
                )
            }
            Bound::Trait(tr) => {
                let (_, set) = source
                    .traits
                    .get(tr)
                    .unwrap_or_else(|| panic!("{}::{}: unknown trait {tr}", m.handle, m.name));
                assert!(!set.is_empty(), "no implementors of {tr} found");
                for ty in set {
                    assert!(
                        h.get(ty).is_some(),
                        "{tr} is implemented for unknown type {ty}"
                    );
                }
            }
        }
    }

    // Each name is defined once per handle, so a link names one method.
    let mut seen = BTreeMap::new();
    for m in &source.methods {
        if let Some(other) = seen.insert((m.handle, &m.name), &m.bound) {
            assert_eq!(
                other, &m.bound,
                "{}::{} is defined for both {other:?} and {:?}",
                m.handle, m.name, m.bound
            );
        }
    }

    // A private helper sharing a public method's name would capture its doc links.
    for (handle, name) in &source.private {
        assert!(
            !seen.contains_key(&(*handle, name)),
            "the private {handle}::{name} shares a name with a public handle method"
        );
    }

    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/handle_methods");
    let update = std::env::var_os("UPDATE_HANDLE_METHODS").is_some();
    let mut stale = Vec::new();
    let mut undocumented = Vec::new();
    let mut expected_files = BTreeSet::new();

    for (class, c) in h.classes.iter().enumerate() {
        let expected = source.render(class);
        if !expected.is_empty() && !c.documented {
            undocumented.push(c.name.clone());
        }
        let file = dir.join(format!("{}.md", c.name));
        expected_files.insert(file.clone());
        let current = fs::read_to_string(&file).ok().map(|s| s.replace('\r', ""));
        if current.as_deref() != Some(expected.as_str()) {
            if update {
                fs::create_dir_all(&dir).unwrap();
                fs::write(&file, &expected).unwrap();
            }
            stale.push(c.name.clone());
        }
    }
    for entry in fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if !expected_files.contains(&path) {
            if update {
                fs::remove_file(&path).unwrap();
            }
            stale.push(path.display().to_string());
        }
    }

    assert!(
        undocumented.is_empty(),
        "types with handle methods need a doc summary first: {undocumented:?}"
    );
    assert!(
        update || stale.is_empty(),
        "handle method docs out of date for {stale:?}; rerun with UPDATE_HANDLE_METHODS=1"
    );
}
