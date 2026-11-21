//! Builds a Skyrim SE file from scratch: nodes, a textured quad and extra data.
//!
//! cargo run --example create

use std::{env, error::Error, fs::File};

use nifly::{
    extra_data::{BsxBitflags, ExtraBsxFlags, ExtraString, NiExtraData},
    prelude::*,
};

fn main() -> Result<(), Box<dyn Error>> {
    // A new file holds only a root node.
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let root = nif.root_index().ok_or("no root node")?;

    // A node under the root, and another under that one.
    let body = nif
        .add_node(
            "Body",
            Transform::from_translation(vec3(0.0, 0.0, 5.0)),
            Some(root),
        )
        .ok_or("couldn't add node")?;
    let arm = nif
        .edit::<NiNode>(body)
        .ok_or("not a node")?
        .add_child("Arm", Transform::from_scale(0.5));

    // A quad, added under the root and moved under the arm.
    let vertices = [
        vec3(-1.0, -1.0, 0.0),
        vec3(1.0, -1.0, 0.0),
        vec3(1.0, 1.0, 0.0),
        vec3(-1.0, 1.0, 0.0),
    ];
    let triangles = [[0, 1, 2], [0, 2, 3]];
    let uvs = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];
    let quad = nif
        .create_shape("Quad", &vertices, &triangles, Some(&uvs), None)
        .ok_or("couldn't create shape")?;
    let mut shape = nif.edit::<NiShape>(quad).ok_or("not a shape")?;
    shape.calc_normals(None);
    shape.set_texture_slot(0, "textures/example_d.dds");
    shape.set_parent(Some(arm));

    // Extra data on the root.
    let mut root = nif.edit::<NiNode>(root).ok_or("not a node")?;
    root.assign_extra_data(ExtraBsxFlags(BsxBitflags::COMPLEX));
    root.assign_extra_data(ExtraString("Prn", "SHIELD"));

    let out = env::temp_dir().join("create_out.nif");
    nif.save(File::create(&out)?, None)?;
    println!("Saved {}", out.display());

    // Read it back and walk the tree.
    let saved = NifFile::open(&out, None)?;
    for obj in saved.tree() {
        println!("  {} \"{}\"", obj.block_name(), name(obj));
    }
    Ok(())
}

/// The block's name, or nothing if its type has none.
fn name(obj: &NiObject) -> String {
    let name = match (obj.downcast::<NiObjectNet>(), obj.downcast::<NiExtraData>()) {
        (Some(o), _) => o.name(),
        (_, Some(e)) => e.name(),
        _ => return String::new(),
    };
    name.to_str_lossy().into_owned()
}
