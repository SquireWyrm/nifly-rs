//! Converts a mesh between Skyrim LE and Skyrim SE.
//!
//! cargo run --example convert [-- <path>]

use std::{env, error::Error, fs::File, path::Path};

use nifly::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).unwrap_or("assets/Bones.nif".into());
    let mut nif = NifFile::open(&path, None)?;

    let (target, suffix) = match nif.version() {
        v if v == NifVersion::SKYRIM => (NifVersion::SKYRIM_SE, "SE"),
        v if v == NifVersion::SKYRIM_SE => (NifVersion::SKYRIM, "LE"),
        v => return Err(format!("{path} is {v:?}, not Skyrim LE or SE").into()),
    };

    println!("Before: {:?}", shape_types(&nif));
    let report = nif.optimize_for(target, None).ok_or("conversion failed")?;
    println!("After:  {:?}", shape_types(&nif));

    if report.duplicates_renamed {
        println!("Renamed shapes with duplicate names");
    }
    for (what, shapes) in [
        ("Vertex colors removed", &report.vertex_colors_removed),
        ("Normals removed", &report.normals_removed),
        ("Partitions triangulated", &report.partitions_triangulated),
        ("Tangents added", &report.tangents_added),
        ("Parallax removed", &report.parallax_removed),
    ] {
        if !shapes.is_empty() {
            println!("{what}: {shapes:?}");
        }
    }

    let stem = Path::new(&path).file_stem().ok_or("no file name")?;
    let out = env::temp_dir().join(format!("{}_{suffix}.nif", stem.to_string_lossy()));
    nif.save(File::create(&out)?, None)?;
    println!("Saved {}", out.display());
    Ok(())
}

/// Each shape's name and block type.
fn shape_types(nif: &NifFile) -> Vec<String> {
    nif.shapes()
        .map(|s| format!("{} {}", s.block_name(), s.name().to_str_lossy()))
        .collect()
}
