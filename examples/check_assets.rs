//! Loads every NIF file under the given paths and runs generic consistency checks on each,
//! for testing the bindings against real game assets that can't be committed to the repo.
//!
//! cargo run --release --example check_assets -- [paths...] [--verbose]
//!
//! Paths can be files or directories (searched recursively) and default to `external_assets`.
//! `.nif`, `.bto` and `.btr` files are checked; `.btr` (terrain) files load with
//! `LoadOptions::is_terrain`. Files are checked in parallel and reported in path order. Exits with
//! status 1 if any check fails.
//!
//! Failures are things the bindings or nifly should never do (e.g. a save that doesn't reload,
//! or two APIs that disagree). Warnings are unusual but legal content (e.g. unknown block types).
//!
//! For saves (without optimizing or sorting) that aren't byte-identical to the input, it reports
//! what changed: 20.2.0.7 files are split into header sections, blocks and footer and compared
//! section by section, and known causes are recognized (texture paths nifly normalizes on load,
//! string table reordering and the resulting index renumbering, empty refs nifly drops, and missing
//! strings written as `""`). Anything else is listed by block type with an example of the changed
//! bytes. Other versions report the first differing byte.
//! Use `--release`: debug builds also run the bindings' `debug_assert!`s, which are slow on big
//! files.

use std::{
    any::Any,
    collections::{BTreeMap, HashMap, HashSet},
    env, fs,
    io::Cursor,
    panic::{self, AssertUnwindSafe},
    path::{Path, PathBuf},
    process, ptr,
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

use rayon::prelude::*;

use nifly::{
    bstr::BStr,
    extra_data::NiExtraData,
    file::{LoadOptions, NifFile, NifVersion, SaveOptions},
    geometry::{NiShape, TextureSource},
    nodes::NiNode,
    object3d::{Transform, Vec3},
    objects::NiObject,
};

const NIF_EXTENSIONS: [&str; 3] = ["nif", "bto", "btr"];

/// Saves with neither option, so block order and contents should survive unchanged.
const KEEP_EVERYTHING: SaveOptions = SaveOptions {
    optimize: false,
    sort_blocks: false,
};

#[derive(Default)]
struct Report {
    failures: Vec<(&'static str, String)>,
    warnings: Vec<(&'static str, String)>,
    /// What a save without optimizing or sorting changed, if it wasn't byte-identical.
    changes: Vec<(String, String)>,
    version: Option<NifVersion>,
    blocks: usize,
    byte_identical: bool,
}

impl Report {
    fn fail(&mut self, kind: &'static str, detail: impl Into<String>) {
        self.failures.push((kind, detail.into()));
    }

    fn warn(&mut self, kind: &'static str, detail: impl Into<String>) {
        self.warnings.push((kind, detail.into()));
    }
}

#[derive(Default)]
struct Stats {
    files: usize,
    passed: usize,
    warned: usize,
    failed: usize,
    blocks: usize,
    byte_identical: usize,
    failure_kinds: BTreeMap<&'static str, usize>,
    warning_kinds: BTreeMap<&'static str, usize>,
    change_kinds: BTreeMap<String, usize>,
    versions: BTreeMap<String, usize>,
}

impl Stats {
    fn add(&mut self, report: &Report) {
        self.files += 1;
        self.blocks += report.blocks;
        self.byte_identical += usize::from(report.byte_identical);
        if !report.failures.is_empty() {
            self.failed += 1;
        } else if !report.warnings.is_empty() {
            self.warned += 1;
        } else {
            self.passed += 1;
        }
        // Count each kind once per file.
        for kind in report.failures.iter().map(|(k, _)| *k).collect::<HashSet<_>>() {
            *self.failure_kinds.entry(kind).or_default() += 1;
        }
        for kind in report.warnings.iter().map(|(k, _)| *k).collect::<HashSet<_>>() {
            *self.warning_kinds.entry(kind).or_default() += 1;
        }
        for kind in report.changes.iter().map(|(k, _)| k).collect::<HashSet<_>>() {
            *self.change_kinds.entry(kind.clone()).or_default() += 1;
        }
        if let Some(v) = report.version {
            *self.versions.entry(version_name(v)).or_default() += 1;
        }
    }
}

fn main() {
    let mut verbose = false;
    let mut roots = Vec::new();
    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-v" | "--verbose" => verbose = true,
            "-h" | "--help" => {
                println!("usage: check_assets [paths...] [--verbose]   (default path: external_assets)");
                return;
            }
            _ => roots.push(PathBuf::from(arg)),
        }
    }
    if roots.is_empty() {
        roots.push(PathBuf::from("external_assets"));
    }

    let mut files = Vec::new();
    for root in &roots {
        if !root.exists() {
            eprintln!("{} does not exist", root.display());
            process::exit(2);
        }
        collect_nifs(root, &mut files);
    }
    files.sort();
    if files.is_empty() {
        eprintln!("no .nif, .bto or .btr files found");
        process::exit(2);
    }
    eprintln!("checking {} files", files.len());

    // Panics are reported per file instead of printed by the default hook.
    panic::set_hook(Box::new(|_| {}));

    // Files are checked in parallel; reports are printed afterwards in path order.
    let start = Instant::now();
    let done = AtomicUsize::new(0);
    let reports: Vec<Report> = files
        .par_iter()
        .map(|path| {
            let report = match panic::catch_unwind(AssertUnwindSafe(|| check_file(path))) {
                Ok(report) => report,
                Err(payload) => {
                    let mut report = Report::default();
                    report.fail("panic", panic_message(&*payload));
                    report
                }
            };
            let n = done.fetch_add(1, Ordering::Relaxed) + 1;
            if n.is_multiple_of(1000) {
                eprintln!("  {n}/{} ({:.0?})", files.len(), start.elapsed());
            }
            report
        })
        .collect();

    let mut stats = Stats::default();
    for (path, report) in files.iter().zip(&reports) {
        stats.add(report);
        print_report(path, report, verbose);
    }

    print_summary(&stats, start.elapsed());
    if stats.failed > 0 {
        process::exit(1);
    }
}

fn collect_nifs(path: &Path, out: &mut Vec<PathBuf>) {
    if path.is_dir() {
        let Ok(entries) = fs::read_dir(path) else {
            eprintln!("can't read {}", path.display());
            return;
        };
        for entry in entries.flatten() {
            collect_nifs(&entry.path(), out);
        }
    } else if extension(path).is_some_and(|e| NIF_EXTENSIONS.contains(&e.as_str())) {
        out.push(path.to_owned());
    }
}

fn extension(path: &Path) -> Option<String> {
    path.extension().map(|e| e.to_string_lossy().to_lowercase())
}

fn print_report(path: &Path, report: &Report, verbose: bool) {
    let status = if !report.failures.is_empty() {
        "FAIL"
    } else if !report.warnings.is_empty() {
        "warn"
    } else {
        "ok"
    };
    if status == "FAIL" || verbose {
        println!("{status:4} {}", path.display());
        for (kind, detail) in &report.failures {
            println!("       fail {kind}: {detail}");
        }
        if verbose {
            for (kind, detail) in &report.warnings {
                println!("       warn {kind}: {detail}");
            }
            for (kind, detail) in &report.changes {
                println!("       save changed {kind}: {detail}");
            }
        }
    }
}

fn print_summary(stats: &Stats, elapsed: std::time::Duration) {
    println!();
    println!(
        "{} files, {} blocks in {:.1?}: {} ok, {} with warnings, {} failed",
        stats.files, stats.blocks, elapsed, stats.passed, stats.warned, stats.failed
    );
    println!(
        "{} of {} files saved back byte-identical (optimize and sort disabled)",
        stats.byte_identical, stats.files
    );

    println!("\nVersions:");
    for (version, count) in &stats.versions {
        println!("  {count:6}  {version}");
    }
    if !stats.failure_kinds.is_empty() {
        println!("\nFailures (files):");
        for (kind, count) in &stats.failure_kinds {
            println!("  {count:6}  {kind}");
        }
    }
    if !stats.warning_kinds.is_empty() {
        println!("\nWarnings (files, see --verbose):");
        for (kind, count) in &stats.warning_kinds {
            println!("  {count:6}  {kind}");
        }
    }
    if !stats.change_kinds.is_empty() {
        println!("\nWhat changed in saves that weren't byte-identical (files, see --verbose):");
        let mut kinds: Vec<_> = stats.change_kinds.iter().collect();
        kinds.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        for (kind, count) in kinds {
            println!("  {count:6}  {kind}");
        }
    }
}

fn version_name(v: NifVersion) -> String {
    let presets = [
        (NifVersion::MORROWIND, "Morrowind"),
        (NifVersion::OBLIVION, "Oblivion"),
        (NifVersion::FALLOUT_3, "Fallout 3"),
        (NifVersion::SKYRIM, "Skyrim"),
        (NifVersion::SKYRIM_SE, "Skyrim SE"),
        (NifVersion::FALLOUT_4, "Fallout 4"),
        (NifVersion::FALLOUT_76, "Fallout 76"),
        (NifVersion::STARFIELD, "Starfield"),
    ];
    let [a, b, c, d] = v.file_parts();
    let game = presets.iter().find(|(p, _)| *p == v).map_or("", |(_, name)| name);
    format!("{a}.{b}.{c}.{d} user {} stream {} {game}", v.user, v.stream)
}

fn panic_message(payload: &(dyn Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_owned()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "(non-string panic)".to_owned()
    }
}

// ---------------------------------------------------------------------------------------------
// Checks
// ---------------------------------------------------------------------------------------------

fn check_file(path: &Path) -> Report {
    let mut report = Report::default();

    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) => {
            report.fail("read", e.to_string());
            return report;
        }
    };
    let options = LoadOptions {
        is_terrain: extension(path).as_deref() == Some("btr"),
    };
    let nif = match NifFile::load(bytes.as_slice(), Some(options)) {
        Ok(nif) => nif,
        Err(e) => {
            report.fail("load", e.to_string());
            return report;
        }
    };
    report.version = Some(nif.version());
    report.blocks = nif.blocks().count();

    check_blocks(&nif, &mut report);
    check_nodes(&nif, &mut report);
    check_shapes(&nif, &mut report);
    check_round_trip(&bytes, options, &mut report);
    report
}

fn check_blocks(nif: &NifFile, r: &mut Report) {
    if !nif.is_valid() {
        r.fail("invalid", "loaded but is_valid() is false");
    }
    if nif.has_unknown() {
        let unknown: HashSet<_> = nif
            .blocks()
            .map(|b| b.block_name())
            .filter(|n| *n == "NiUnknown")
            .collect();
        r.warn("unknown blocks", format!("{unknown:?}"));
    }

    let count = nif.blocks().count();
    for (i, block) in nif.blocks().enumerate() {
        if nif.block_index(block) != Some(i) {
            r.fail("block_index", format!("block {i} reports {:?}", nif.block_index(block)));
        }
        if !nif.contains(block) {
            r.fail("contains", format!("block {i} not contained"));
        }
    }
    if nif.block(count).is_some() {
        r.fail("block", "block(count) is Some");
    }

    if nif.root_node().is_none() && count > 0 {
        r.warn("no root node", "");
    }

    let mut seen = HashSet::new();
    for block in nif.tree() {
        if !seen.insert(ptr::from_ref(block)) {
            r.fail("tree", "visited a block twice");
            break;
        }
    }
    let unreachable = count - seen.len().min(count);
    if nif.root_node().is_some() && unreachable > 0 {
        r.warn("unreachable blocks", format!("{unreachable} of {count}"));
    }
}

fn check_nodes(nif: &NifFile, r: &mut Report) {
    let mut name_counts: HashMap<&BStr, usize> = HashMap::new();
    for node in nif.nodes() {
        *name_counts.entry(node.name()).or_default() += 1;
    }

    for node in nif.nodes().views() {
        let index = node.index();
        if has_nan(&node.transform()) {
            r.warn("NaN in transform", format!("block {index}"));
        }

        // Child refs can be empty in game files. nifly drops empty refs when saving.
        let resolved = node.children().count();
        if node.child_count() > resolved {
            r.warn(
                "empty child refs",
                format!("block {index}: {} refs, {resolved} children", node.child_count()),
            );
        }

        for child in node.children().views() {
            match child.parent() {
                None => r.fail("parent_node", format!("child of block {index} has no parent")),
                Some(parent) if parent.index() != index => {
                    r.warn("shared children", format!("a child of block {index} has another parent first"));
                }
                Some(_) => {}
            }
        }

        // Name lookups only find the first node with a name, so only check unique names.
        if name_counts[node.name()] == 1 {
            let name = node.name();
            let same = nif
                .node_transform_to_parent(name)
                .is_some_and(|t| transforms_identical(&t, &node.transform()));
            if !same {
                r.fail("node_transform_to_parent", format!("block {index}"));
            }
            match nif.node_transform_to_global(name) {
                None => r.fail("node_transform_to_global", format!("block {index}: None")),
                Some(global) => {
                    let expected = global_via_parents(nif, node.get());
                    if !transforms_close(&global, &expected) {
                        r.fail(
                            "node_transform_to_global",
                            format!("block {index}: nifly {global:?}, parent chain {expected:?}"),
                        );
                    }
                }
            }
        }
    }
}

fn check_shapes(nif: &NifFile, r: &mut Report) {
    let is_starfield = nif.version().stream >= 172;
    let mut all_sse = true;

    for (index, shape) in nif.shapes().indexed() {
        all_sse &= nif.view_of(shape).unwrap().is_sse_compatible();

        // texture_slot agrees with texture_paths for texture set slots.
        let paths = nif.view_of(shape).unwrap().texture_paths();
        for (slot, path) in paths.iter().enumerate() {
            if let Some((TextureSource::TextureSet, slot_path)) = nif.view_of(shape).unwrap().texture_slot(slot)
                && slot_path != *path
            {
                r.fail("texture_slot", format!("shape {index} slot {slot}"));
            }
        }

        // Bone names and IDs agree (Starfield binds bones by name instead).
        let bones = nif.view_of(shape).unwrap().bone_list();
        let ids = nif.view_of(shape).unwrap().bone_ids();
        if !is_starfield {
            if bones.len() != ids.len() {
                r.warn("bone list/ids", format!("shape {index}: {} names, {} ids", bones.len(), ids.len()));
            } else {
                for (name, id) in bones.iter().zip(&ids) {
                    match nif.block(*id).and_then(|b| b.downcast::<NiNode>()) {
                        Some(bone) if bone.name() == *name => {}
                        _ => r.warn("bone list/ids", format!("shape {index}: {name} isn't block {id}")),
                    }
                }
            }
        }

        // Weights are sorted, in range, and roughly normalized per vertex.
        let mut sums: HashMap<u16, f32> = HashMap::new();
        for b in 0..bones.len() {
            let weights = nif.view_of(shape).unwrap().bone_weights(b);
            if !weights.windows(2).all(|w| w[0].vertex < w[1].vertex) {
                r.fail("bone weights", format!("shape {index} bone {b} not sorted by vertex"));
            }
            for w in &weights {
                if !(w.weight > 0.0 && w.weight <= 1.001) {
                    r.warn("weight out of range", format!("shape {index} bone {b}: {}", w.weight));
                }
                *sums.entry(w.vertex).or_default() += w.weight;
            }
        }
        let unnormalized = sums.values().filter(|&&s| (s - 1.0).abs() > 0.02).count();
        if unnormalized > 0 {
            r.warn("unnormalized weights", format!("shape {index}: {unnormalized} vertices"));
        }
    }

    if nif.is_all_sse_compatible() != all_sse {
        r.fail("is_all_sse_compatible", "disagrees with checking each shape");
    }
}

fn check_round_trip(original: &[u8], options: LoadOptions, r: &mut Report) {
    let load = |bytes: &[u8]| NifFile::load(bytes, Some(options));
    let save = |nif: &mut NifFile, opts: SaveOptions| -> Result<Vec<u8>, String> {
        let mut cur = Cursor::new(Vec::new());
        nif.save(&mut cur, Some(opts)).map_err(|e| e.to_string())?;
        Ok(cur.into_inner())
    };

    // Without optimizing or sorting, everything the bindings can read survives a round trip.
    let Ok(mut fresh) = load(original) else { return };
    let before = summary(&fresh);
    let first = match save(&mut fresh, KEEP_EVERYTHING) {
        Ok(bytes) => bytes,
        Err(e) => return r.fail("save", e),
    };
    r.byte_identical = first == original;
    if !r.byte_identical {
        r.changes = describe_changes(original, &first);
    }

    let mut reloaded = match load(&first) {
        Ok(nif) => nif,
        Err(e) => return r.fail("reload", e.to_string()),
    };
    let after = summary(&reloaded);
    if let Some(diff) = first_difference(&before, &after) {
        r.fail("round trip", diff);
    }
    match save(&mut reloaded, KEEP_EVERYTHING) {
        Ok(second) if second != first => r.fail("save not stable", "second save differs from the first"),
        Ok(_) => {}
        Err(e) => r.fail("save", e),
    }

    // With the default options, the output reloads and saving it again changes nothing.
    let Ok(mut fresh) = load(original) else { return };
    let optimized = match save(&mut fresh, SaveOptions::default()) {
        Ok(bytes) => bytes,
        Err(e) => return r.fail("save (default)", e),
    };
    match load(&optimized) {
        Ok(mut nif) => {
            if !nif.is_valid() {
                r.fail("reload (default)", "not valid");
            }
            match save(&mut nif, SaveOptions::default()) {
                Ok(again) if again != optimized => {
                    r.fail("save not stable (default)", "second save differs from the first");
                }
                Ok(_) => {}
                Err(e) => r.fail("save (default)", e),
            }
        }
        Err(e) => r.fail("reload (default)", e.to_string()),
    }
}

/// Everything the bindings can read, as comparable lines.
fn summary(nif: &NifFile) -> Vec<String> {
    let mut out = vec![format!("version {:?}", nif.version())];
    out.extend(nif.blocks().map(|b| b.block_name().to_owned()));
    for node in nif.nodes() {
        // Resolved children, not child_count: nifly drops empty child refs when saving.
        out.push(format!(
            "node {:?} {:?} {:#x} children {}",
            node.name(),
            node.transform(),
            node.raw_flags(),
            nif.view_of(node).unwrap().children().count()
        ));
        for extra in nif.view_of(node).unwrap().extra_data() {
            out.push(format!("  extra {} {}", extra.block_name(), extra_name(extra)));
        }
    }
    for shape in nif.blocks_of::<NiShape>() {
        out.push(format!("shape {:?} {:?} {:#x}", shape.name(), shape.transform(), shape.raw_flags()));
        out.push(format!("  textures {:?}", nif.view_of(shape).unwrap().texture_paths()));
        let bones = nif.view_of(shape).unwrap().bone_list();
        out.push(format!("  bones {bones:?} {:?}", nif.view_of(shape).unwrap().bone_ids()));
        for b in 0..bones.len() {
            out.push(format!("  weights {b} {:?}", nif.view_of(shape).unwrap().bone_weights(b)));
        }
    }
    out
}

fn extra_name(obj: &NiObject) -> String {
    obj.downcast::<NiExtraData>().map_or(String::new(), |e| format!("{:?}", e.name()))
}

fn first_difference(before: &[String], after: &[String]) -> Option<String> {
    for (i, (a, b)) in before.iter().zip(after).enumerate() {
        if a != b {
            // Show the line's start and the text around the first differing character.
            let at = a.bytes().zip(b.bytes()).take_while(|(x, y)| x == y).count();
            let window = |s: &str| {
                let start = s.floor_char_boundary(at.saturating_sub(40));
                let end = s.ceil_char_boundary((at + 60).min(s.len()));
                s[start..end].to_owned()
            };
            let head: String = a.chars().take(40).collect();
            return Some(format!("line {i} ({head}...): ...{}... -> ...{}...", window(a), window(b)));
        }
    }
    (before.len() != after.len())
        .then(|| format!("{} summary lines -> {}", before.len(), after.len()))
}

// ---------------------------------------------------------------------------------------------
// Transforms
// ---------------------------------------------------------------------------------------------

/// Global transform composed in Rust by walking `parent_node`, to check nifly's
/// `GetNodeTransformToGlobal` against.
fn global_via_parents(nif: &NifFile, node: &NiNode) -> Transform {
    let mut global = node.transform();
    let mut parent = nif.view_of(node).and_then(|v| v.parent());
    let mut depth = 0;
    while let Some(p) = parent {
        global = p.transform() * global;
        parent = p.parent();
        depth += 1;
        if depth > 1000 {
            break; // cycle guard for malformed files
        }
    }
    global
}

fn transform_floats(t: &Transform) -> [f32; 13] {
    let [r0, r1, r2] = t.rotation.rows;
    let tr = t.translation;
    [tr.x, tr.y, tr.z, r0.x, r0.y, r0.z, r1.x, r1.y, r1.z, r2.x, r2.y, r2.z, t.scale]
}

fn has_nan(t: &Transform) -> bool {
    transform_floats(t).iter().any(|f| f.is_nan())
}

/// Exact equality, but NaN equals NaN (some game files have NaN rotations).
fn transforms_identical(a: &Transform, b: &Transform) -> bool {
    let (a, b) = (transform_floats(a), transform_floats(b));
    a.iter().zip(&b).all(|(x, y)| x == y || (x.is_nan() && y.is_nan()))
}

/// Approximate equality, where NaN equals NaN.
fn transforms_close(a: &Transform, b: &Transform) -> bool {
    let close = |x: f32, y: f32| {
        (x.is_nan() && y.is_nan()) || (x - y).abs() <= 1e-3 * (1.0 + x.abs().max(y.abs()))
    };
    let vec_close = |p: Vec3, q: Vec3| close(p.x, q.x) && close(p.y, q.y) && close(p.z, q.z);
    vec_close(a.translation, b.translation)
        && (0..3).all(|i| vec_close(a.rotation.rows[i], b.rotation.rows[i]))
        && close(a.scale, b.scale)
}

// ---------------------------------------------------------------------------------------------
// Byte-level diff of a save
// ---------------------------------------------------------------------------------------------

/// A 20.2.0.7 (Bethesda) NIF file split into its sections.
struct Layout<'a> {
    export_info: &'a [u8],
    block_types: Vec<&'a [u8]>,
    type_indices: Vec<u16>,
    strings: Vec<&'a [u8]>,
    groups: &'a [u8],
    blocks: Vec<&'a [u8]>,
    footer: &'a [u8],
}

impl Layout<'_> {
    fn type_of(&self, block: usize) -> String {
        // The high bit of a type index is a flag, not part of the index.
        let index = usize::from(self.type_indices[block] & 0x7FFF);
        self.block_types.get(index).map_or("?".into(), |t| String::from_utf8_lossy(t).into_owned())
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let slice = self.bytes.get(self.pos..self.pos.checked_add(n)?)?;
        self.pos += n;
        Some(slice)
    }
    fn u8(&mut self) -> Option<u8> {
        Some(self.take(1)?[0])
    }
    fn u16(&mut self) -> Option<u16> {
        Some(u16::from_le_bytes(self.take(2)?.try_into().ok()?))
    }
    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }
    /// A string with a u8 length (Bethesda export info).
    fn short_string(&mut self) -> Option<&'a [u8]> {
        let len = self.u8()?;
        self.take(usize::from(len))
    }
    /// A string with a u32 length.
    fn sized_string(&mut self) -> Option<&'a [u8]> {
        let len = self.u32()?;
        self.take(usize::try_from(len).ok()?)
    }
}

/// Parses the header layout of 20.2.0.7 files (Skyrim, Fallout 4, ...). `None` for other
/// versions, or if the sections don't add up to the file's length.
fn parse_layout(bytes: &[u8]) -> Option<Layout<'_>> {
    if !bytes.starts_with(b"Gamebryo File Format") {
        return None;
    }
    let mut r = Reader { bytes, pos: bytes.iter().position(|&b| b == b'\n')? + 1 };
    if r.u32()? != 0x14020007 {
        return None;
    }
    let _endian = r.u8()?;
    let user = r.u32()?;
    let num_blocks = usize::try_from(r.u32()?).ok()?;

    let export_start = r.pos;
    if user >= 3 {
        let stream = r.u32()?;
        r.short_string()?; // author
        if stream > 130 {
            r.u32()?;
        }
        if stream < 131 {
            r.short_string()?; // process script
        }
        r.short_string()?; // export script
        if stream >= 103 {
            r.short_string()?; // max filepath
        }
    }
    let export_info = &bytes[export_start..r.pos];

    let num_types = r.u16()?;
    let block_types = (0..num_types).map(|_| r.sized_string()).collect::<Option<Vec<_>>>()?;
    let type_indices = (0..num_blocks).map(|_| r.u16()).collect::<Option<Vec<_>>>()?;
    let sizes = (0..num_blocks).map(|_| r.u32()).collect::<Option<Vec<_>>>()?;
    let num_strings = r.u32()?;
    let _max_string_length = r.u32()?;
    let strings = (0..num_strings).map(|_| r.sized_string()).collect::<Option<Vec<_>>>()?;
    let num_groups = usize::try_from(r.u32()?).ok()?;
    let groups = r.take(num_groups.checked_mul(4)?)?;

    let blocks = sizes
        .iter()
        .map(|&size| r.take(usize::try_from(size).ok()?))
        .collect::<Option<Vec<_>>>()?;

    // Footer: a root count and that many root block indices.
    let footer_start = r.pos;
    let num_roots = usize::try_from(r.u32()?).ok()?;
    r.take(num_roots.checked_mul(4)?)?;
    if r.pos != bytes.len() {
        return None;
    }

    Some(Layout {
        export_info,
        block_types,
        type_indices,
        strings,
        groups,
        blocks,
        footer: &bytes[footer_start..],
    })
}

/// What differs between `original` and `saved`, by section.
fn describe_changes(original: &[u8], saved: &[u8]) -> Vec<(String, String)> {
    let size = format!("{} -> {} bytes", original.len(), saved.len());
    let (Some(a), Some(b)) = (parse_layout(original), parse_layout(saved)) else {
        let at = original.iter().zip(saved).take_while(|(x, y)| x == y).count();
        return vec![("unparsed layout".into(), format!("{size}, first difference at byte {at}"))];
    };

    let mut changes = Vec::new();
    let lossy = |s: &[u8]| String::from_utf8_lossy(s).into_owned();
    let examples = |items: &[&[u8]]| {
        let shown: Vec<_> = items.iter().take(5).map(|s| lossy(s)).collect();
        let more = items.len().saturating_sub(5);
        if more > 0 { format!("{shown:?} and {more} more") } else { format!("{shown:?}") }
    };

    if a.export_info != b.export_info {
        changes.push(("header: export info".into(), size.clone()));
    }
    if a.block_types != b.block_types {
        let (sa, sb): (HashSet<_>, HashSet<_>) =
            (a.block_types.iter().collect(), b.block_types.iter().collect());
        let kind = if sa == sb { "header: block types reordered" } else { "header: block types changed" };
        changes.push((kind.into(), String::new()));
    }

    if a.strings != b.strings {
        let (sa, sb): (HashSet<&[u8]>, HashSet<&[u8]>) =
            (a.strings.iter().copied().collect(), b.strings.iter().copied().collect());
        let mut removed: Vec<_> = a.strings.iter().copied().filter(|s| !sb.contains(s)).collect();
        let mut added: Vec<_> = b.strings.iter().copied().filter(|s| !sa.contains(s)).collect();
        removed.dedup();
        added.dedup();
        let duplicates_removed = a.strings.len() - sa.len() > b.strings.len() - sb.len();

        if removed.is_empty() && added.is_empty() && !duplicates_removed {
            changes.push(("header: strings reordered".into(), format!("{} strings", a.strings.len())));
        }
        if duplicates_removed {
            let dropped = (a.strings.len() - sa.len()) - (b.strings.len() - sb.len());
            changes.push(("header: duplicate strings removed".into(), format!("{dropped}")));
        }
        if !removed.is_empty() {
            changes.push(("header: unused strings removed".into(), examples(&removed)));
        }
        if !added.is_empty() {
            changes.push(("header: strings added".into(), examples(&added)));
        }
    }
    if a.groups != b.groups {
        changes.push(("header: groups".into(), String::new()));
    }

    if a.blocks.len() != b.blocks.len() {
        changes.push(("block count".into(), format!("{} -> {}", a.blocks.len(), b.blocks.len())));
    } else {
        // Group differing blocks by what explains the difference, then by type.
        let mut renumbered: BTreeMap<String, usize> = BTreeMap::new();
        let mut texture_example = None;
        let mut texture_sets = 0;
        let mut empty_refs: BTreeMap<String, usize> = BTreeMap::new();
        let mut none_to_empty: BTreeMap<String, usize> = BTreeMap::new();
        // (blocks, total size change, example of changed bytes)
        let mut other: BTreeMap<String, (usize, i64, Option<String>)> = BTreeMap::new();
        for (i, (x, y)) in a.blocks.iter().zip(&b.blocks).enumerate() {
            if x == y {
                continue;
            }
            let block_type = a.type_of(i);
            if string_indices_renumbered(x, y, &a.strings, &b.strings, false) {
                *renumbered.entry(block_type).or_default() += 1;
            } else if string_indices_renumbered(x, y, &a.strings, &b.strings, true) {
                *none_to_empty.entry(block_type).or_default() += 1;
            } else if let Some((old, new)) = changed_texture_path(&block_type, x, y) {
                texture_sets += 1;
                texture_example.get_or_insert(format!("{:?} -> {:?}", lossy(old), lossy(new)));
            } else if let Some(dropped) = empty_refs_dropped(x, y) {
                *empty_refs.entry(block_type).or_default() += dropped;
            } else {
                let entry = other.entry(block_type).or_default();
                entry.0 += 1;
                entry.1 += y.len() as i64 - x.len() as i64;
                if entry.2.is_none() && x.len() == y.len() {
                    entry.2 = Some(first_bytes_changed(x, y));
                }
            }
        }
        if !none_to_empty.is_empty() {
            let types: Vec<_> = none_to_empty.iter().map(|(t, n)| format!("{t} x{n}")).collect();
            changes.push(("block data: missing strings written as \"\"".into(), types.join(", ")));
        }
        if !empty_refs.is_empty() {
            let types: Vec<_> = empty_refs.iter().map(|(t, n)| format!("{t}: {n} refs")).collect();
            changes.push(("block data: empty refs dropped".into(), types.join(", ")));
        }
        if !renumbered.is_empty() {
            let types: Vec<_> = renumbered.iter().map(|(t, n)| format!("{t} x{n}")).collect();
            changes.push(("block data: string indices renumbered".into(), types.join(", ")));
        }
        if let Some(example) = texture_example {
            changes.push((
                "block data: texture paths normalized on load".into(),
                format!("{texture_sets} texture sets, e.g. {example}"),
            ));
        }
        for (block_type, (count, delta, example)) in other {
            let example = example.map_or(String::new(), |e| format!(", e.g. {e}"));
            changes.push((
                format!("block data: {block_type}"),
                format!("{count} blocks, {delta:+} bytes{example}"),
            ));
        }
    }
    if a.footer != b.footer {
        changes.push(("footer: roots".into(), String::new()));
    }

    if changes.is_empty() {
        // Same sections, so the difference is in a length or count field that was re-encoded.
        changes.push(("other header fields".into(), size));
    }
    changes
}

/// Whether two blocks of the same length differ only in header string indices that point to the
/// same string in each file's table (which happens when the string table is reordered).
///
/// With `none_as_empty`, an old index of `0xFFFFFFFF` (no string) also matches a new index of an
/// empty string: nifly writes empty strings as a reference to a `""` table entry.
fn string_indices_renumbered(
    x: &[u8],
    y: &[u8],
    xs: &[&[u8]],
    ys: &[&[u8]],
    none_as_empty: bool,
) -> bool {
    if x.len() != y.len() {
        return false;
    }
    let read = |b: &[u8], at: usize| u32::from_le_bytes(b[at..at + 4].try_into().unwrap()) as usize;
    let mut covered_until = 0;
    for p in (0..x.len()).filter(|&p| x[p] != y[p]) {
        if p < covered_until {
            continue;
        }
        // Find a 4-byte field containing `p` whose old and new values name the same string.
        let explained = (p.saturating_sub(3)..=p).filter(|&s| s + 4 <= x.len()).any(|s| {
            let (i, j) = (read(x, s), read(y, s));
            let renumbered = i != j && i < xs.len() && j < ys.len() && xs[i] == ys[j];
            let none_to_empty =
                none_as_empty && i == u32::MAX as usize && j < ys.len() && ys[j].is_empty();
            let same = renumbered || none_to_empty;
            if same {
                covered_until = s + 4;
            }
            same
        });
        if !explained {
            return false;
        }
    }
    true
}

/// For a `BSShaderTextureSet` (a count, then length-prefixed paths), the first path that differs.
fn changed_texture_path<'a>(block_type: &str, x: &'a [u8], y: &'a [u8]) -> Option<(&'a [u8], &'a [u8])> {
    if block_type != "BSShaderTextureSet" {
        return None;
    }
    let paths = |b: &'a [u8]| -> Option<Vec<&'a [u8]>> {
        let mut r = Reader { bytes: b, pos: 0 };
        let count = r.u32()?;
        let paths = (0..count).map(|_| r.sized_string()).collect::<Option<Vec<_>>>()?;
        (r.pos == b.len()).then_some(paths)
    };
    let (px, py) = (paths(x)?, paths(y)?);
    if px.len() != py.len() {
        return None;
    }
    px.into_iter().zip(py).find(|(a, b)| a != b)
}

/// If `y` is `x` with some empty refs (`0xFFFFFFFF`) removed, how many. nifly drops empty refs
/// from ref lists when saving, so the block shrinks by 4 bytes per ref.
fn empty_refs_dropped(x: &[u8], y: &[u8]) -> Option<usize> {
    const EMPTY: [u8; 4] = [0xFF; 4];
    let shrink = x.len().checked_sub(y.len())?;
    if shrink == 0 || shrink % 4 != 0 {
        return None;
    }
    let count = |b: &[u8]| {
        let mut n = 0;
        let mut i = 0;
        while i + 4 <= b.len() {
            if b[i..i + 4] == EMPTY {
                n += 1;
                i += 4;
            } else {
                i += 1;
            }
        }
        n
    };
    let dropped = shrink / 4;
    (count(x) >= count(y) + dropped).then_some(dropped)
}

/// The first few changed bytes of two same-length blocks, as `@offset old->new`.
fn first_bytes_changed(x: &[u8], y: &[u8]) -> String {
    let diffs: Vec<_> = (0..x.len())
        .filter(|&i| x[i] != y[i])
        .take(4)
        .map(|i| format!("@{i} {:02x}->{:02x}", x[i], y[i]))
        .collect();
    diffs.join(" ")
}
