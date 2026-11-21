//! Loads a file, prints its scene graph and shapes, edits it and saves a copy.
//!
//! cargo run --example simple [-- <path>]

use std::{collections::BTreeMap, env, error::Error, fs::File};

use nifly::{extra_data::ExtraString, prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).unwrap_or("assets/Cube.nif".into());
    let mut nif = NifFile::open(&path, None)?;

    // Every block, counted by type.
    let v = nif.version();
    println!(
        "{path}: version {:#x}, user {}, stream {}",
        v.file, v.user, v.stream
    );
    let mut counts = BTreeMap::new();
    for block in nif.blocks() {
        *counts.entry(block.block_name()).or_insert(0) += 1;
    }
    for (name, count) in &counts {
        println!("  {count:3} {name}");
    }

    // The scene graph, from the root node down.
    println!("\nScene graph:");
    let root = nif.root_node().ok_or("no root node")?;
    print_tree(&nif, root.get(), 1);

    // Each shape's geometry, shader and textures.
    println!("\nShapes:");
    for shape in nif.shapes().views() {
        println!(
            "  {}: {} vertices, {} triangles, shader {}",
            shape.name().to_str_lossy(),
            shape.vertices().map_or(0, |v| v.len()),
            shape.triangles().map_or(0, |t| t.len()),
            shape.shader().map_or("(none)", |s| s.get().block_name()),
        );
        for path in shape.texture_paths().iter().filter(|p| !p.is_empty()) {
            println!("    texture: {path}");
        }
    }

    // Move the root up, retexture the first shape and tag the root with extra data.
    let mut root = nif.root_node_mut().ok_or("no root node")?;
    root.set_transform(Transform::from_translation(vec3(0.0, 0.0, 10.0)));
    root.assign_extra_data(ExtraString("Prn", "WEAPON"));
    if let Some(shape) = nif.shapes().indices().next() {
        let mut shape = nif.edit::<NiShape>(shape).ok_or("not a shape")?;
        shape.set_texture_slot(0, "textures/example_d.dds");
    }

    // Save a copy and read it back.
    let out = env::temp_dir().join("simple_out.nif");
    nif.save(File::create(&out)?, None)?;
    let saved = NifFile::open(&out, None)?;
    println!("\nSaved {}", out.display());
    for shape in saved.shapes().views() {
        let textures: Vec<_> = shape
            .texture_paths()
            .into_iter()
            .filter(|p| !p.is_empty())
            .collect();
        println!("  {}: {textures:?}", shape.name().to_str_lossy());
    }
    Ok(())
}

fn print_tree(nif: &NifFile, obj: &NiAvObject, depth: usize) {
    let t = obj.transform().translation;
    println!(
        "{:pad$}{} \"{}\" at ({:.2}, {:.2}, {:.2})",
        "",
        obj.block_name(),
        obj.name().to_str_lossy(),
        t.x,
        t.y,
        t.z,
        pad = depth * 2,
    );
    if let Some(node) = obj.downcast::<NiNode>() {
        for child in nif.view_of(node).unwrap().children() {
            print_tree(nif, child, depth + 1);
        }
    }
}
